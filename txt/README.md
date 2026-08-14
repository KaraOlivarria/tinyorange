# txt

Collaborative typesetting for open textbooks. This is the beginning of the
tool specified in [docs/txt/spec.md](../docs/txt/spec.md) and
[docs/txt/format.md](../docs/txt/format.md).

**Status: v0 scaffold.** Lives inside the founding repo for now; extraction
to its own repository (with the full Apache-2.0 LICENSE file) happens when
it grows real users. Licensed Apache-2.0 per
[docs/licensing.md](../docs/licensing.md).

## What works today

- **`.tx` parsing**: TOML frontmatter (required `id`/`kind`/`title`,
  `standards`, arbitrary extras) and nested `::: kind {#id key="value"}`
  directive blocks, with line-accurate errors.
- **`txt check`** on a book directory or single file:
  - `book.toml` presence + format version
  - block kinds validated against the format vocabulary
  - required `{#id}` on pedagogical blocks
  - figures: required non-empty `alt:` text, required `src`, and the
    source file must exist
  - `@ref` cross-references must resolve; ids must be unique book-wide
  - `audience` attribute values validated
- **`txt parse`**: dump a file's document model as JSON.

```console
$ cargo run -q -p txt-cli -- check ../examples/algebra-1-demo
✓ 3 file(s), 34 block(s), 0 errors

$ cargo run -q -p txt-cli -- parse ../examples/algebra-1-demo/SUMMARY.tx
{ "frontmatter": { "id": "summary", ... }, "nodes": [ ... ] }
```

The demo book doubles as the golden test:
`cargo test` parses `examples/algebra-1-demo` and requires a clean check.

## Layout

```
crates/
├── txt-core   parser, document model, check rules (library, no I/O in parsing)
└── txt-cli    the `txt` binary
```

## Known v0 limitations (deliberate)

- `:::` markers inside fenced code blocks are not yet ignored.
- Inline markup (math, glossary terms, citations) is carried as raw text;
  only `@refs` are extracted. Full inline parsing arrives with the first
  renderer.
- No renderers yet — `check` and `parse` only.

## Next milestones (spec §4, roadmap Phase 1)

1. HTML renderer (the free digital edition) with audience stripping
2. `txt fmt` canonical formatter (auto-generating missing ids)
3. Typst-vs-LaTeX print benchmark → PDF target
4. Block-aware git merge/diff drivers
