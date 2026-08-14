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
- **`txt build`** — the Typst backend (ADR 0001): compiles a book to PDF
  through the embedded design template, with TeX-subset math translation,
  numbered cross-references resolved in prose, and audience stripping
  (student vs. teacher editions from one source).
- **`txt parse`**: dump a file's document model as JSON.

```console
$ cargo run -q -p txt-cli -- check ../examples/algebra-1-demo
✓ 3 file(s), 34 block(s), 0 errors

$ cargo run -q -p txt-cli -- build ../examples/algebra-1-demo --audience student
✓ ../examples/algebra-1-demo/build/student/algebra-1-demo-student.pdf

$ cargo run -q -p txt-cli -- build ../examples/algebra-1-demo --audience teacher
✓ ../examples/algebra-1-demo/build/teacher/algebra-1-demo-teacher.pdf
```

`--target typst` writes the generated sources without compiling (no Typst
needed); PDF compilation finds `typst` via `$TXT_TYPST`, `$PATH`, or
`~/.local/bin`. A build refuses to run on a book that fails `txt check`.

The demo book doubles as the golden test:
`cargo test` parses `examples/algebra-1-demo` and requires a clean check.

## Layout

```
crates/
├── txt-core          parser, document model, schema, check rules
├── txt-render-typst  Typst backend: template.typ + doc-model→Typst renderer
└── txt-cli           the `txt` binary
schema/
└── blocks.toml       normative block vocabulary (embedded into txt-core)
```

## Known v0 limitations (deliberate)

- `:::` markers inside fenced code blocks are not yet ignored.
- Math is the v1 TeX subset (format.md §5); unknown commands are build
  errors and the set grows by RFC.
- Cross-references render as resolved text ("Example 1"), not hyperlinks.
- `[text](url)` links in prose are not yet inline-parsed.
- No HTML/EPUB renderers yet.

## Next milestones (spec §4, roadmap Phase 1)

1. HTML renderer (the free digital edition) sharing the same doc model
2. `txt fmt` canonical formatter (auto-generating missing ids)
3. PDF/UA tagging evaluation on the Typst backend (ADR 0001 revisit trigger)
4. Block-aware git merge/diff drivers
