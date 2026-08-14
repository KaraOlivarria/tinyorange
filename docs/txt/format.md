# The `.tx` Source Format

Version 0.2 (draft). `.tx` is txt's plain-text source format for books:
a strict, semantic superset of CommonMark with typed blocks, required
metadata, and stable addressing.

Design position: **familiar enough that a Markdown user is productive in
minutes; strict enough that a build is a real guarantee.**

Normative artifacts (this document is the readable companion):

- [`grammar.ebnf`](grammar.ebnf) — the surface grammar
- [`txt/schema/blocks.toml`](../../txt/schema/blocks.toml) — the block
  vocabulary: allowed kinds, attributes, nesting, and required fields,
  embedded and enforced by txt-core
- [ADR 0001](adr/0001-tx-language-typst-backend.md) — why `.tx` is a new
  language while the print engine is an embedded backend (Typst first)

---

## 1. Principles

1. **CommonMark base.** Paragraphs, emphasis, lists, tables (GFM), links
   work as educators expect.
2. **Typed blocks carry pedagogy.** Definitions, worked examples, exercises,
   objectives, warnings are first-class typed structures, not styled quotes.
3. **Everything addressable.** Every block has a stable ID — for review
   comments, translation anchoring, semantic diff, and cross-references
   that survive reorganization.
4. **Strictness where it protects students.** Missing alt text, dangling
   references, or unlabeled answer content are parse/build errors.
5. **One canonical formatting.** `txt fmt` normalizes whitespace and
   attribute order so diffs show meaning, not style.

## 2. File Anatomy

Every `.tx` file: TOML frontmatter + body.

```
+++
id = "ch-linear-eq"            # stable, never renamed after publication
kind = "chapter"               # chapter | section | exercises | glossary…
title = "Linear equations"
standards = ["CCSS.MATH.8.EE.C.7"]
+++

body…
```

## 3. Typed Blocks

Fenced with `:::` (directive syntax), typed, attributed. IDs are required on
pedagogical blocks (`txt fmt` auto-generates missing ones as
`{kind}-{slug}-{short-hash}` so authors aren't burdened).

```markdown
::: objective {#obj-solve-one-var}
Solve linear equations in one variable, including equations with
coefficients represented by letters.
:::

::: definition {#def-linear-equation term="linear equation"}
A **linear equation** is an equation in which each term is either a
constant or the product of a constant and a single variable.
:::

::: example {#ex-two-step difficulty=2}
Solve $3x + 4 = 19$.

::: solution
Subtract 4 from both sides, then divide by 3: $x = 5$.
:::
:::

::: exercise {#exr-practice-1 difficulty=1 standards="CCSS.MATH.8.EE.C.7"}
Solve $5x - 2 = 13$.

::: answer {audience="teacher"}
$x = 3$.
:::
:::

::: figure {#fig-slope src="figures/slope.svg"}
alt: A line on a coordinate plane rising from left to right, crossing
  the y-axis at 2, with a right triangle showing a rise of 3 and a run of 1.
caption: The slope of a line is rise over run.
credit: Texture illustration team, CC BY 4.0.
:::
```

Core block vocabulary (v1): `objective`, `definition`, `theorem`, `example`,
`solution`, `exercise`, `answer`, `hint`, `figure`, `table`, `note`,
`warning`, `activity`, `vocabulary`, `summary`, `citation`, `source`.
The vocabulary — including each kind's allowed attributes, required
fields, and nesting constraints (e.g., `solution` only inside `example`/
`exercise`/`activity`) — is defined by
[`txt/schema/blocks.toml`](../../txt/schema/blocks.toml). Books cannot
invent block types (design consistency + renderer guarantees); the schema
evolves by editorial-board RFC.

## 4. Audience Flags

`audience="teacher"` (answers, teaching notes) is stripped from student
builds and included in teacher-edition builds. Publishing a student target
that would include unstripped teacher content is a build error. This single
mechanism yields student book, teacher edition, and answer keys from one
source.

## 5. Inline Semantics

The v1 inline set: `**strong**`, `*emphasis*`, `` `code` ``, math, cross-
references, citations, glossary spans, and index spans. Headings (`#`,
sentence case), bullet lists (`- `), and numbered lists (`1. `) structure
text runs between blocks.

Backslash escapes a special character to its literal self:
`\$` `\*` `\[` `\]` `` \` `` `\@` `\#` `\_` `\\`. Currency is the common
case — write `costs \$6`, reserve bare `$…$` for math.

- Math: `$…$` / `$$…$$`, a **TeX-compatible subset** that is parsed and
  translated by renderers — never passed through to any engine. The v1
  command set is `\frac`, `\dfrac`, `\cdot`, `\quad`, `\qquad`, and
  `^`/`_` scripts; an unknown command is a build error, and the set grows
  by RFC, not by renderer whim. Output: MathML (web/EPUB) + engine-native
  math (PDF).
- Cross-references: `@fig-slope`, `@exr-practice-1` — rendered with the
  right label ("Figure 3.2") per target language and design spec; dangling
  refs are build errors.
- Glossary terms: `[term]{.gloss}` links to the `definition` block that
  declared it.
- Citations: `[@smith2024]` against a CSL-JSON bibliography per book;
  claims-needing-source lint (advisory) flags bare empirical claims.
- Index entries: `[photosynthesis]{.index}` — print index is generated,
  never hand-built.

## 5a. Book Structure (`SUMMARY.tx`)

`SUMMARY.tx` (frontmatter kind `summary`) is the book's canonical spine:
a nested bullet list of `[title](relative/path.tx)` links whose order is
the book's order for every build target. Headings (`#`) group entries
into parts. Files not reachable from `SUMMARY.tx` are not part of the
book (checked; orphans are an error in book builds).

## 6. Standards Alignment

`standards = [...]` attributes (frontmatter or per-block) reference a
versioned standards registry (Common Core, NGSS, state frameworks as
datasets in a shared repo). `txt check` validates codes; the platform
renders coverage maps ("which sections address NGSS MS-LS1-6?") — a major
adoption argument for districts.

## 7. Translation Model

- Source language lives in `chapters/`; translations in
  `locale/<bcp47>/…` mirroring structure.
- Translation units anchor to **block IDs + source-content hash**: when the
  English block changes, the translated block is flagged stale, not
  clobbered. `txt translate status` reports coverage and staleness per
  locale.
- Numbers, cross-reference labels, and design strings localize via the
  design spec's string tables, not per-book.

## 8. Stability & Evolution

- `format = "1"` declared in `book.toml`; parsers reject newer majors.
- Additive changes (new block attributes) are minor; anything that changes
  parse results is major and ships with `txt migrate`.
- The grammar (EBNF) and block schema (JSON Schema) are normative artifacts
  in the txt repo; this document is the readable companion.

## 9. Why not just Markdown / LaTeX / XML? (anticipated question)

- **Markdown alone:** no typed pedagogy, no required a11y, no stable IDs —
  everything that makes review, translation, and multi-target builds work.
- **LaTeX:** visual/programmatic mixing, unreviewable diffs for
  non-technical educators, effectively single-target.
- **DocBook/XML (e.g., PreTeXt):** semantically strong but hostile to
  hand-authoring; `.tx` targets the same semantic rigor at Markdown-level
  ergonomics. (PreTeXt, CNXML/OpenStax, and iBooks-era formats are prior
  art worth a formal review during v1 design.)
