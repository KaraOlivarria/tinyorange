use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};
use std::process::Command;
use txt_core::{check_book, parse as parse_tx, Diagnostic, Document, Severity};
use txt_render_typst::{render_book, Audience, BookMeta, TEMPLATE_TYP};

#[derive(Parser)]
#[command(
    name = "txt",
    version,
    about = "Collaborative typesetting for open textbooks — Texture"
)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Parse and check a book directory (with book.toml) or a single .tx file
    Check { path: PathBuf },
    /// Parse one .tx file and print its document model as JSON
    Parse { file: PathBuf },
    /// Build a book. Targets: typst (generate sources), pdf (via Typst)
    Build {
        book: PathBuf,
        #[arg(long, default_value = "pdf")]
        target: String,
        /// student strips teacher-only blocks; teacher includes them
        #[arg(long, default_value = "student")]
        audience: String,
        /// Output directory (default: <book>/build/<audience>)
        #[arg(long, short = 'o')]
        out: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    match Cli::parse().cmd {
        Cmd::Check { path } => cmd_check(&path),
        Cmd::Parse { file } => cmd_parse(&file),
        Cmd::Build { book, target, audience, out } => cmd_build(&book, &target, &audience, out),
    }
}

fn cmd_parse(file: &Path) -> Result<()> {
    let src = std::fs::read_to_string(file)
        .with_context(|| format!("reading {}", file.display()))?;
    let doc = parse_tx(&src).map_err(|e| anyhow::anyhow!("{}: {e}", file.display()))?;
    println!("{}", serde_json::to_string_pretty(&doc)?);
    Ok(())
}

// ---------------------------------------------------------------- check

fn cmd_check(path: &Path) -> Result<()> {
    let (root, files) = if path.is_file() {
        (path.parent().map(Path::to_path_buf).unwrap_or_default(), vec![path.to_path_buf()])
    } else if path.is_dir() {
        read_book_toml(path)?;
        let mut found = Vec::new();
        collect_tx(path, &mut found)?;
        found.sort();
        if found.is_empty() {
            bail!("no .tx files found under {}", path.display());
        }
        (path.to_path_buf(), found)
    } else {
        bail!("{}: no such file or directory", path.display());
    };

    let (parsed, problems) = parse_and_check(&files)?;
    report(&root, &parsed, &files, problems)
}

fn parse_and_check(
    files: &[PathBuf],
) -> Result<(Vec<(PathBuf, Document)>, Vec<(PathBuf, Diagnostic)>)> {
    let mut parsed: Vec<(PathBuf, Document)> = Vec::new();
    let mut problems: Vec<(PathBuf, Diagnostic)> = Vec::new();
    for file in files {
        let src = std::fs::read_to_string(file)
            .with_context(|| format!("reading {}", file.display()))?;
        match parse_tx(&src) {
            Ok(doc) => parsed.push((file.clone(), doc)),
            Err(e) => problems.push((file.clone(), Diagnostic::error(e.line, "parse", e.message))),
        }
    }
    problems.extend(check_book(&parsed));
    problems.sort_by(|a, b| (&a.0, a.1.line).cmp(&(&b.0, b.1.line)));
    Ok((parsed, problems))
}

fn report(
    root: &Path,
    parsed: &[(PathBuf, Document)],
    files: &[PathBuf],
    problems: Vec<(PathBuf, Diagnostic)>,
) -> Result<()> {
    for (file, d) in &problems {
        let rel = file.strip_prefix(root).unwrap_or(file);
        let level = match d.severity {
            Severity::Error => "error",
            Severity::Warning => "warning",
        };
        eprintln!("{}:{}: {level}[{}]: {}", rel.display(), d.line, d.rule, d.message);
    }
    let n_blocks: usize = parsed.iter().map(|(_, d)| d.blocks().len()).sum();
    let n_errors = problems.iter().filter(|(_, d)| d.severity == Severity::Error).count();
    if n_errors == 0 {
        println!("✓ {} file(s), {} block(s), 0 errors", files.len(), n_blocks);
        Ok(())
    } else {
        println!("✗ {} file(s), {} block(s), {} error(s)", files.len(), n_blocks, n_errors);
        std::process::exit(1);
    }
}

// ---------------------------------------------------------------- build

struct BookToml {
    id: String,
    title: String,
    subtitle: Option<String>,
    edition: String,
}

fn read_book_toml(dir: &Path) -> Result<BookToml> {
    let path = dir.join("book.toml");
    if !path.exists() {
        bail!("{} is not a book: no book.toml found", dir.display());
    }
    let raw = std::fs::read_to_string(&path)
        .with_context(|| format!("reading {}", path.display()))?;
    let value: toml::Value = toml::from_str(&raw)
        .with_context(|| format!("invalid TOML in {}", path.display()))?;
    match value.get("format").and_then(toml::Value::as_str) {
        Some("1") => {}
        Some(other) => bail!("unsupported format version `{other}` (this txt understands format \"1\")"),
        None => bail!("book.toml is missing required key `format`"),
    }
    let book = value
        .get("book")
        .and_then(toml::Value::as_table)
        .context("book.toml is missing the [book] table")?;
    let get = |key: &str| book.get(key).and_then(toml::Value::as_str).map(String::from);
    Ok(BookToml {
        id: get("id").context("book.toml [book] is missing `id`")?,
        title: get("title").context("book.toml [book] is missing `title`")?,
        subtitle: get("subtitle"),
        edition: get("edition").unwrap_or_else(|| "draft".into()),
    })
}

/// The spine: `[title](path.tx)` links from SUMMARY.tx, in order.
fn spine_from_summary(doc: &Document) -> Vec<String> {
    let mut spine = Vec::new();
    for t in doc.texts() {
        for line in t.content.lines() {
            let mut rest = line;
            while let Some(pos) = rest.find("](") {
                let after = &rest[pos + 2..];
                let Some(end) = after.find(')') else { break };
                let target = &after[..end];
                if target.ends_with(".tx") {
                    spine.push(target.to_string());
                }
                rest = &after[end + 1..];
            }
        }
    }
    spine
}

fn cmd_build(book_dir: &Path, target: &str, audience: &str, out: Option<PathBuf>) -> Result<()> {
    let audience = match audience {
        "student" => Audience::Student,
        "teacher" => Audience::Teacher,
        other => bail!("unknown audience `{other}` (student | teacher)"),
    };
    if !matches!(target, "pdf" | "typst") {
        bail!("unknown target `{target}` (pdf | typst)");
    }

    let meta = read_book_toml(book_dir)?;
    let summary_path = book_dir.join("SUMMARY.tx");
    let summary_src = std::fs::read_to_string(&summary_path)
        .with_context(|| format!("reading {} (every book needs a SUMMARY.tx spine)", summary_path.display()))?;
    let summary = parse_tx(&summary_src)
        .map_err(|e| anyhow::anyhow!("{}: {e}", summary_path.display()))?;

    let mut files = vec![summary_path.clone()];
    for rel in spine_from_summary(&summary) {
        let p = book_dir.join(&rel);
        if !p.exists() {
            bail!("SUMMARY.tx lists `{rel}` but it does not exist");
        }
        files.push(p);
    }

    // Orphans: .tx files in the book that the spine doesn't reach (format §5a).
    let mut all_tx = Vec::new();
    collect_tx(book_dir, &mut all_tx)?;
    for f in &all_tx {
        if !files.contains(f) {
            bail!("`{}` is not reachable from SUMMARY.tx — add it to the spine or remove it", f.display());
        }
    }

    // A build is a guarantee: refuse to build a book that doesn't check.
    let (parsed, problems) = parse_and_check(&files)?;
    if !problems.is_empty() {
        report(book_dir, &parsed, &files, problems)?;
    }

    let book_meta = BookMeta {
        title: meta.title.clone(),
        subtitle: meta.subtitle.clone(),
        edition: format!("Edition {}", meta.edition),
    };
    let rendered = render_book(&book_meta, &parsed, audience)
        .map_err(|e| anyhow::anyhow!("render: {e}"))?;

    let out_dir = out.unwrap_or_else(|| book_dir.join("build").join(audience.as_str()));
    std::fs::create_dir_all(out_dir.join("assets"))
        .with_context(|| format!("creating {}", out_dir.display()))?;
    std::fs::write(out_dir.join("template.typ"), TEMPLATE_TYP)?;
    std::fs::write(out_dir.join("main.typ"), &rendered.main_typ)?;
    for asset in &rendered.assets {
        std::fs::copy(&asset.source, out_dir.join(&asset.dest)).with_context(|| {
            format!("copying {} → {}", asset.source.display(), asset.dest)
        })?;
    }

    if target == "typst" {
        println!("✓ typst sources written to {}", out_dir.display());
        return Ok(());
    }

    let typst = find_typst()?;
    let pdf_name = format!("{}-{}.pdf", meta.id, audience.as_str());
    let status = Command::new(&typst)
        .args(["compile", "main.typ", &pdf_name])
        .current_dir(&out_dir)
        .status()
        .with_context(|| format!("running {}", typst.display()))?;
    if !status.success() {
        bail!("typst compile failed (sources kept in {})", out_dir.display());
    }
    println!("✓ {}", out_dir.join(pdf_name).display());
    Ok(())
}

/// Locates the Typst binary: $TXT_TYPST, then $PATH, then ~/.local/bin.
fn find_typst() -> Result<PathBuf> {
    if let Ok(p) = std::env::var("TXT_TYPST") {
        return Ok(PathBuf::from(p));
    }
    if let Some(paths) = std::env::var_os("PATH") {
        for dir in std::env::split_paths(&paths) {
            let candidate = dir.join("typst");
            if candidate.is_file() {
                return Ok(candidate);
            }
        }
    }
    if let Some(home) = std::env::var_os("HOME") {
        let candidate = Path::new(&home).join(".local/bin/typst");
        if candidate.is_file() {
            return Ok(candidate);
        }
    }
    bail!(
        "typst binary not found — install it (https://typst.app) or set TXT_TYPST; \
         `txt build --target typst` works without it"
    )
}

fn collect_tx(dir: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    for entry in std::fs::read_dir(dir).with_context(|| format!("reading {}", dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if name.starts_with('.') {
            continue;
        }
        if path.is_dir() {
            if name == "target" || name == "build" {
                continue;
            }
            collect_tx(&path, out)?;
        } else if path.extension().is_some_and(|e| e == "tx") {
            out.push(path);
        }
    }
    Ok(())
}
