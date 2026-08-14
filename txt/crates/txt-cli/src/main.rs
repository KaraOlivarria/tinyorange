use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};
use txt_core::{check_book, parse as parse_tx, Diagnostic, Document, Severity};

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
}

fn main() -> Result<()> {
    match Cli::parse().cmd {
        Cmd::Check { path } => cmd_check(&path),
        Cmd::Parse { file } => cmd_parse(&file),
    }
}

fn cmd_parse(file: &Path) -> Result<()> {
    let src = std::fs::read_to_string(file)
        .with_context(|| format!("reading {}", file.display()))?;
    let doc = parse_tx(&src).map_err(|e| anyhow::anyhow!("{}: {e}", file.display()))?;
    println!("{}", serde_json::to_string_pretty(&doc)?);
    Ok(())
}

fn cmd_check(path: &Path) -> Result<()> {
    let (root, files) = if path.is_file() {
        (path.parent().map(Path::to_path_buf).unwrap_or_default(), vec![path.to_path_buf()])
    } else if path.is_dir() {
        validate_book_toml(path)?;
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

    let mut parsed: Vec<(PathBuf, Document)> = Vec::new();
    let mut problems: Vec<(PathBuf, Diagnostic)> = Vec::new();
    for file in &files {
        let src = std::fs::read_to_string(file)
            .with_context(|| format!("reading {}", file.display()))?;
        match parse_tx(&src) {
            Ok(doc) => parsed.push((file.clone(), doc)),
            Err(e) => problems.push((
                file.clone(),
                Diagnostic::error(e.line, "parse", e.message),
            )),
        }
    }
    problems.extend(check_book(&parsed));
    problems.sort_by(|a, b| (&a.0, a.1.line).cmp(&(&b.0, b.1.line)));

    for (file, d) in &problems {
        let rel = file.strip_prefix(&root).unwrap_or(file);
        let level = match d.severity {
            Severity::Error => "error",
            Severity::Warning => "warning",
        };
        eprintln!("{}:{}: {level}[{}]: {}", rel.display(), d.line, d.rule, d.message);
    }

    let n_blocks: usize = parsed.iter().map(|(_, d)| d.blocks().len()).sum();
    let n_errors = problems
        .iter()
        .filter(|(_, d)| d.severity == Severity::Error)
        .count();
    if n_errors == 0 {
        println!("✓ {} file(s), {} block(s), 0 errors", files.len(), n_blocks);
        Ok(())
    } else {
        println!("✗ {} file(s), {} block(s), {} error(s)", files.len(), n_blocks, n_errors);
        std::process::exit(1);
    }
}

fn validate_book_toml(dir: &Path) -> Result<()> {
    let path = dir.join("book.toml");
    if !path.exists() {
        bail!("{} is not a book: no book.toml found", dir.display());
    }
    let raw = std::fs::read_to_string(&path)
        .with_context(|| format!("reading {}", path.display()))?;
    let value: toml::Value = toml::from_str(&raw)
        .with_context(|| format!("invalid TOML in {}", path.display()))?;
    match value.get("format").and_then(toml::Value::as_str) {
        Some("1") => Ok(()),
        Some(other) => bail!("unsupported format version `{other}` (this txt understands format \"1\")"),
        None => bail!("book.toml is missing required key `format`"),
    }
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
            if name == "target" {
                continue;
            }
            collect_tx(&path, out)?;
        } else if path.extension().is_some_and(|e| e == "tx") {
            out.push(path);
        }
    }
    Ok(())
}
