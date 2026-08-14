# txt

Collaborative typesetting for open textbooks. This is the beginning of the
tool specified in [docs/txt/spec.md](../docs/txt/spec.md) and
[docs/txt/format.md](../docs/txt/format.md).

**Status: v0 scaffold.** Lives inside the founding repo for now; extraction
to its own repository (with the full Apache-2.0 LICENSE file) happens when
it grows real users. Licensed Apache-2.0 per
[docs/licensing.md](../docs/licensing.md).

## What works today

- **`.tx` parsing**: TOML frontmatter, nested `::: kind {#id key="value"}`
  directive blocks, body structure (headings, lists, display math), and
  the inline grammar (strong/emphasis, code, math, `@refs`, citations,
  glossary spans, backslash escapes) — with line-accurate errors.
- **`txt check`**, driven by the normative schema
  ([`schema/blocks.toml`](schema/blocks.toml)): block vocabulary, required
  `{#id}`s, allowed/required attributes, nesting constraints (`solution`
  only inside `example`/`exercise`/…), required figure `alt:` text, figure
  sources on disk, `@ref` resolution, book-wide id uniqueness, audience
  values.
- **`txt build`** — two backends from one document model (ADR 0001):
  - `--target html`: the free digital edition. Static, semantic HTML +
    native MathML, hyperlinked cross-references, collapsible solutions
    and hints, light/dark brand CSS, no JavaScript, no tracking.
  - `--target pdf`: the Typst print backend — embedded design template,
    TeX-subset math translation, hyperlinked numbered cross-references.
  - Both strip `audience="teacher"` blocks from student builds: student
    book, teacher edition, and answer keys come from one source.
- **`txt fmt`** — the canonical formatter: one attribute order and
  quoting style so diffs show meaning, plus auto-generated stable ids
  (`{kind}-{slug}-{hash}`) for blocks that need them. `--check` for CI.
- **`txt parse`**: dump a file's document model as JSON.

```console
$ cargo run -q -p txt-cli -- check ../examples/algebra-1-demo
✓ 3 file(s), 34 block(s), 0 errors

$ cargo run -q -p txt-cli -- build ../examples/algebra-1-demo --target html
✓ ../examples/algebra-1-demo/build/student/html/index.html

$ cargo run -q -p txt-cli -- build ../examples/algebra-1-demo --target pdf --audience teacher
✓ ../examples/algebra-1-demo/build/teacher/pdf/algebra-1-demo-teacher.pdf

$ cargo run -q -p txt-cli -- fmt --check ../examples
✓ 3 file(s) checked, 0 would change
```

`--target typst` writes generated sources without compiling; PDF
compilation finds `typst` via `$TXT_TYPST`, `$PATH`, or `~/.local/bin`.
A build refuses to run on a book that fails `txt check`.

The demo book doubles as the golden test: CI runs the full suite, checks
canonical formatting, and builds every target × audience on every push.

## Layout

```
crates/
├── txt-core          parser, document model, schema, checks, math tree, fmt
├── txt-render-typst  print backend: template.typ + doc-model→Typst
├── txt-render-html   digital edition: semantic HTML + MathML + brand CSS
└── txt-cli           the `txt` binary
schema/
└── blocks.toml       normative block vocabulary (embedded into txt-core)
```

Math is parsed once in txt-core into a semantic tree; each backend emits
its native form (Typst math, MathML). No TeX ever passes through raw.

## Known v0 limitations (deliberate)

- `:::` markers inside fenced code blocks are not yet ignored.
- Math is the v1 TeX subset (format.md §5); unknown commands are build
  errors and the set grows by RFC.
- `[text](url)` links in prose are not yet inline-parsed.
- No EPUB renderer yet; glossary terms render italic, not linked.

## Next milestones (spec §4, roadmap Phase 1)

1. EPUB 3 target reusing the HTML renderer
2. PDF/UA tagging evaluation on the Typst backend (ADR 0001 revisit trigger)
3. Block-aware git merge/diff drivers
4. `txt preview` (live-reload local server over the HTML backend)
