//! HTML backend for txt: the free digital edition. Static, semantic
//! HTML + MathML — offline-capable, no accounts, no tracking, no
//! JavaScript. Accessibility is structural: real headings, landmarks,
//! required alt text, MathML for screen readers, and a skip link.

pub mod mathml;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use txt_core::{
    directive_fields, ref_targets, structure, Asset, Audience, Block, BlockSchema, BodyElement,
    BookMeta, Document, Inline, Node, RefTarget,
};

pub const STYLE_CSS: &str = include_str!("style.css");

pub struct Page {
    pub filename: String,
    pub html: String,
}

pub struct RenderedSite {
    pub pages: Vec<Page>,
    pub assets: Vec<Asset>,
}

#[derive(Debug)]
pub struct RenderError {
    pub file: PathBuf,
    pub message: String,
}

impl std::fmt::Display for RenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.file.display(), self.message)
    }
}

impl std::error::Error for RenderError {}

struct Ctx<'a> {
    audience: Audience,
    targets: &'a BTreeMap<String, RefTarget>,
    assets: Vec<Asset>,
    file: PathBuf,
    base: PathBuf,
    /// Frontmatter id of the page being rendered (for same-page anchors).
    page_id: String,
}

pub fn render_book(
    meta: &BookMeta,
    files: &[(PathBuf, Document)],
    audience: Audience,
) -> Result<RenderedSite, RenderError> {
    let targets = ref_targets(files);
    let mut ctx = Ctx {
        audience,
        targets: &targets,
        assets: Vec::new(),
        file: PathBuf::new(),
        base: PathBuf::new(),
        page_id: String::new(),
    };

    let chapters: Vec<&(PathBuf, Document)> = files
        .iter()
        .filter(|(_, d)| d.frontmatter.kind != "summary")
        .collect();

    let mut pages = vec![index_page(meta, &chapters, audience)];
    for (i, (path, doc)) in chapters.iter().enumerate() {
        ctx.file = path.clone();
        ctx.base = path.parent().unwrap_or(Path::new(".")).to_path_buf();
        ctx.page_id = doc.frontmatter.id.clone();

        let mut body = String::new();
        body.push_str(&format!("<h1>{}</h1>\n", esc(&doc.frontmatter.title)));
        render_nodes(&mut ctx, &doc.nodes, &mut body)?;

        let prev = i
            .checked_sub(1)
            .map(|j| nav_link(&chapters[j].1, "‹ Previous"));
        let next = chapters.get(i + 1).map(|c| nav_link(&c.1, "Next ›"));
        let pager = format!(
            "<nav class=\"pager\" aria-label=\"Book navigation\">{}<a href=\"index.html\">Contents</a>{}</nav>",
            prev.unwrap_or_default(),
            next.unwrap_or_default(),
        );

        pages.push(Page {
            filename: format!("{}.html", doc.frontmatter.id),
            html: shell(meta, &doc.frontmatter.title, audience, &format!("{pager}\n<main id=\"main\"><article>\n{body}</article></main>")),
        });
    }

    Ok(RenderedSite { pages, assets: ctx.assets })
}

fn nav_link(doc: &Document, text: &str) -> String {
    format!(
        "<a href=\"{}.html\">{}</a>",
        esc_attr(&doc.frontmatter.id),
        text
    )
}

fn index_page(meta: &BookMeta, chapters: &[&(PathBuf, Document)], audience: Audience) -> Page {
    let mut body = String::new();
    body.push_str("<main id=\"main\">\n");
    body.push_str(&format!("<h1>{}</h1>\n", esc(&meta.title)));
    if let Some(sub) = &meta.subtitle {
        body.push_str(&format!("<p class=\"subtitle\">{}</p>\n", esc(sub)));
    }
    body.push_str(&format!(
        "<p class=\"edition-line\">{} · {} edition · free forever under <a href=\"https://creativecommons.org/licenses/by/4.0/\">CC BY 4.0</a></p>\n",
        esc(&meta.edition),
        audience.as_str(),
    ));
    body.push_str("<nav class=\"contents\" aria-label=\"Contents\"><h2>Contents</h2>\n<ol>\n");
    for (_, doc) in chapters {
        body.push_str(&format!(
            "<li><a href=\"{}.html\">{}</a></li>\n",
            esc_attr(&doc.frontmatter.id),
            esc(&doc.frontmatter.title)
        ));
    }
    body.push_str("</ol>\n</nav>\n</main>");
    Page { filename: "index.html".into(), html: shell(meta, "Contents", audience, &body) }
}

fn shell(meta: &BookMeta, page_title: &str, audience: Audience, content: &str) -> String {
    let issue_link = match &meta.repository {
        Some(repo) => format!(
            " · <a href=\"{}/issues\">Report an issue</a>",
            esc_attr(repo.trim_end_matches('/'))
        ),
        None => String::new(),
    };
    format!(
        "<!doctype html>\n<html lang=\"{lang}\">\n<head>\n<meta charset=\"utf-8\">\n\
         <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n\
         <title>{page_title} · {title}</title>\n\
         <link rel=\"stylesheet\" href=\"style.css\">\n</head>\n<body>\n\
         <a class=\"skip\" href=\"#main\">Skip to content</a>\n\
         <header class=\"site\"><a class=\"book-title\" href=\"index.html\">{title}</a>\
         <span class=\"edition-badge\">{audience} edition</span></header>\n\
         {content}\n\
         <footer class=\"site\">Published by Texture under \
         <a href=\"https://creativecommons.org/licenses/by/4.0/\">CC BY 4.0</a> — \
         free to read, adapt, and share{issue_link} · Built with txt</footer>\n\
         </body>\n</html>\n",
        lang = esc_attr(&meta.language),
        page_title = esc(page_title),
        title = esc(&meta.title),
        audience = audience.as_str(),
    )
}

fn render_nodes(ctx: &mut Ctx, nodes: &[Node], out: &mut String) -> Result<(), RenderError> {
    for node in nodes {
        match node {
            Node::Text(t) => render_text(ctx, &t.content, out)?,
            Node::Block(b) => render_block(ctx, b, out)?,
        }
    }
    Ok(())
}

fn render_text(ctx: &Ctx, content: &str, out: &mut String) -> Result<(), RenderError> {
    for el in structure(content) {
        match el {
            BodyElement::Heading { level, content } => {
                // chapter titles are h1; body headings clamp to h2..h6
                let h = level.clamp(2, 6);
                out.push_str(&format!("<h{h}>{}</h{h}>\n", inlines(ctx, &content)?));
            }
            BodyElement::Paragraph(items) => {
                out.push_str(&format!("<p>{}</p>\n", inlines(ctx, &items)?));
            }
            BodyElement::BulletList(list) => {
                out.push_str("<ul>\n");
                for item in &list {
                    out.push_str(&format!("<li>{}</li>\n", inlines(ctx, item)?));
                }
                out.push_str("</ul>\n");
            }
            BodyElement::OrderedList(list) => {
                out.push_str("<ol>\n");
                for item in &list {
                    out.push_str(&format!("<li>{}</li>\n", inlines(ctx, item)?));
                }
                out.push_str("</ol>\n");
            }
            BodyElement::DisplayMath(src) => {
                let m = mathml::tex_to_mathml(&src, true).map_err(|e| err(ctx, &src, e))?;
                out.push_str(&m);
                out.push('\n');
            }
        }
    }
    Ok(())
}

fn render_block(ctx: &mut Ctx, b: &Block, out: &mut String) -> Result<(), RenderError> {
    if ctx.audience == Audience::Student
        && b.attrs.get("audience").map(String::as_str) == Some("teacher")
    {
        return Ok(());
    }

    let id_attr = match &b.id {
        Some(id) => format!(" id=\"{}\"", esc_attr(id)),
        None => String::new(),
    };
    let label = b
        .id
        .as_ref()
        .and_then(|id| ctx.targets.get(id).map(|t| t.label.clone()));

    match b.kind.as_str() {
        "objective" => {
            let mut body = String::new();
            render_inline_children(ctx, b, &mut body)?;
            out.push_str(&format!("<p class=\"objective\"{id_attr}>{body}</p>\n"));
        }
        "definition" | "theorem" => {
            let mut l = label.unwrap_or_else(|| title_case(&b.kind));
            if let Some(term) = b.attrs.get("term").or_else(|| b.attrs.get("name")) {
                l = format!("{l} — {term}");
            }
            callout(ctx, b, &b.kind.clone(), &l, &id_attr, out)?;
        }
        "note" => callout(ctx, b, "note", "Note", &id_attr, out)?,
        "warning" => callout(ctx, b, "warning", "Watch out", &id_attr, out)?,
        "vocabulary" => callout(ctx, b, "vocabulary", "Vocabulary", &id_attr, out)?,
        "summary" => callout(ctx, b, "summary", "Summary", &id_attr, out)?,
        "activity" => callout(ctx, b, "activity", "Activity", &id_attr, out)?,
        "answer" => callout(ctx, b, "answer", "Answer · teacher edition", &id_attr, out)?,
        "example" => {
            let l = label.unwrap_or_else(|| "Example".into());
            let mut body = String::new();
            render_nodes(ctx, &b.children, &mut body)?;
            out.push_str(&format!(
                "<section class=\"example\"{id_attr}><p class=\"example-label\">{}</p>\n{body}</section>\n",
                esc(&l)
            ));
        }
        "exercise" => {
            let l = label.unwrap_or_else(|| "Exercise".into());
            let difficulty = b
                .attrs
                .get("difficulty")
                .and_then(|d| d.parse::<usize>().ok())
                .map(|d| {
                    format!(
                        "<span class=\"difficulty\" aria-label=\"Difficulty {d} of 3\">{}</span>",
                        "●".repeat(d)
                    )
                })
                .unwrap_or_default();
            let mut body = String::new();
            render_nodes(ctx, &b.children, &mut body)?;
            out.push_str(&format!(
                "<section class=\"exercise\"{id_attr}><p class=\"exercise-label\">{}{difficulty}</p>\n{body}</section>\n",
                esc(&l)
            ));
        }
        "solution" | "hint" => {
            let mut body = String::new();
            render_nodes(ctx, &b.children, &mut body)?;
            out.push_str(&format!(
                "<details class=\"{kind}\"{id_attr}><summary>{}</summary>\n{body}</details>\n",
                title_case(&b.kind),
                kind = b.kind,
            ));
        }
        "figure" => render_figure(ctx, b, label.as_deref(), &id_attr, out)?,
        "citation" | "source" => {}
        _ => render_nodes(ctx, &b.children, out)?,
    }
    Ok(())
}

fn callout(
    ctx: &mut Ctx,
    b: &Block,
    class: &str,
    label: &str,
    id_attr: &str,
    out: &mut String,
) -> Result<(), RenderError> {
    let mut body = String::new();
    render_nodes(ctx, &b.children, &mut body)?;
    out.push_str(&format!(
        "<section class=\"callout {class}\"{id_attr}><p class=\"callout-label\">{}</p>\n{body}</section>\n",
        esc(label)
    ));
    Ok(())
}

/// For blocks whose content is a single prose run (objectives).
fn render_inline_children(ctx: &Ctx, b: &Block, out: &mut String) -> Result<(), RenderError> {
    for node in &b.children {
        if let Node::Text(t) = node {
            for el in structure(&t.content) {
                if let BodyElement::Paragraph(items) = el {
                    out.push_str(&inlines(ctx, &items)?);
                }
            }
        }
    }
    Ok(())
}

fn render_figure(
    ctx: &mut Ctx,
    b: &Block,
    label: Option<&str>,
    id_attr: &str,
    out: &mut String,
) -> Result<(), RenderError> {
    let schema_fields = &BlockSchema::v1().kind("figure").expect("figure in schema").fields;
    let fields = directive_fields(b, schema_fields);
    let src = b
        .attrs
        .get("src")
        .ok_or_else(|| err(ctx, "figure", "missing src (check should have caught this)".into()))?;
    let source = ctx.base.join(src);
    let filename = source
        .file_name()
        .map(|f| f.to_string_lossy().into_owned())
        .unwrap_or_else(|| "figure".into());
    let dest = format!("assets/fig-{:02}-{}", ctx.assets.len() + 1, filename);
    ctx.assets.push(Asset { source, dest: dest.clone() });

    let alt = fields.get("alt").cloned().unwrap_or_default();
    let caption = fields.get("caption").cloned().unwrap_or_default();
    let credit = fields
        .get("credit")
        .map(|c| format!("<span class=\"credit\">{}</span>", esc(c)))
        .unwrap_or_default();
    out.push_str(&format!(
        "<figure{id_attr}><img src=\"{}\" alt=\"{}\">\n<figcaption><span class=\"fig-label\">{}.</span> {}{credit}</figcaption></figure>\n",
        esc_attr(&dest),
        esc_attr(&alt),
        esc(label.unwrap_or("Figure")),
        esc(&caption),
    ));
    Ok(())
}

fn inlines(ctx: &Ctx, items: &[Inline]) -> Result<String, RenderError> {
    let mut out = String::new();
    for item in items {
        match item {
            Inline::Str(s) => out.push_str(&esc(s)),
            Inline::Strong(inner) => {
                out.push_str(&format!("<strong>{}</strong>", inlines(ctx, inner)?))
            }
            Inline::Emph(inner) => out.push_str(&format!("<em>{}</em>", inlines(ctx, inner)?)),
            Inline::Code(c) => out.push_str(&format!("<code>{}</code>", esc(c))),
            Inline::Math(m) => {
                let converted =
                    mathml::tex_to_mathml(m, false).map_err(|e| err(ctx, m, e))?;
                out.push_str(&converted);
            }
            Inline::Ref(id) => match ctx.targets.get(id) {
                Some(t) => {
                    let href = if t.is_block && t.file_id == ctx.page_id {
                        format!("#{id}")
                    } else if t.is_block {
                        format!("{}.html#{id}", t.file_id)
                    } else {
                        format!("{}.html", t.file_id)
                    };
                    out.push_str(&format!(
                        "<a class=\"xref\" href=\"{}\">{}</a>",
                        esc_attr(&href),
                        esc(&t.label)
                    ));
                }
                None => return Err(err(ctx, id, format!("unresolved reference `@{id}`"))),
            },
            Inline::Citation(key) => {
                out.push_str(&format!("<span class=\"citation\">[{}]</span>", esc(key)))
            }
            Inline::Gloss(term) => {
                out.push_str(&format!("<i class=\"gloss\">{}</i>", esc(term)))
            }
        }
    }
    Ok(out)
}

fn err(ctx: &Ctx, context: &str, message: String) -> RenderError {
    RenderError {
        file: ctx.file.clone(),
        message: format!("{message} (in `{context}`)"),
    }
}

fn title_case(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn esc_attr(s: &str) -> String {
    esc(s).replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use txt_core::parse;

    fn render(body: &str, audience: Audience) -> RenderedSite {
        let src = format!(
            "+++\nid = \"ch-t\"\nkind = \"chapter\"\ntitle = \"Test chapter\"\n+++\n{body}"
        );
        let doc = parse(&src).unwrap();
        let meta = BookMeta {
            title: "Test Book".into(),
            subtitle: None,
            edition: "Edition 0".into(),
            language: "en-US".into(),
            repository: Some("https://example.org/repo".into()),
        };
        render_book(&meta, &[(PathBuf::from("t.tx"), doc)], audience).unwrap()
    }

    #[test]
    fn produces_index_and_chapter_pages() {
        let site = render("Hello world.\n", Audience::Student);
        assert_eq!(site.pages.len(), 2);
        assert_eq!(site.pages[0].filename, "index.html");
        assert_eq!(site.pages[1].filename, "ch-t.html");
        assert!(site.pages[0].html.contains("<a href=\"ch-t.html\">Test chapter</a>"));
        assert!(site.pages[1].html.contains("<p>Hello world.</p>"));
        assert!(site.pages[1].html.contains("lang=\"en-US\""));
        assert!(site.pages[1].html.contains("Report an issue"));
    }

    #[test]
    fn math_is_mathml_and_answers_strip() {
        let body = "::: exercise {#e1 difficulty=2}\nSolve $3x = 6$.\n\n::: answer {audience=\"teacher\"}\n$x = 2$.\n:::\n:::\n";
        let student = render(body, Audience::Student);
        let page = &student.pages[1].html;
        assert!(page.contains("<math><mrow><mn>3</mn><mi>x</mi>"), "{page}");
        assert!(!page.contains("answer"), "{page}");
        assert!(page.contains("aria-label=\"Difficulty 2 of 3\""), "{page}");
        let teacher = render(body, Audience::Teacher);
        assert!(teacher.pages[1].html.contains("Answer · teacher edition"));
    }

    #[test]
    fn refs_become_same_page_anchors() {
        let body = "::: example {#ex-a}\nFirst.\n:::\n\nSee @ex-a.\n";
        let site = render(body, Audience::Student);
        let page = &site.pages[1].html;
        assert!(page.contains("<a class=\"xref\" href=\"#ex-a\">Example 1</a>"), "{page}");
        assert!(page.contains("id=\"ex-a\""), "{page}");
    }

    #[test]
    fn solutions_are_collapsible() {
        let body = "::: example {#e}\nQ.\n\n::: solution\nA.\n:::\n:::\n";
        let site = render(body, Audience::Student);
        let page = &site.pages[1].html;
        assert!(page.contains("<details class=\"solution\""), "{page}");
        assert!(page.contains("<summary>Solution</summary>"), "{page}");
    }
}
