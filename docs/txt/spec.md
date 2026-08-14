# txt — Technical Specification

Version 0.1 (draft). `txt` is Texture's open-source, git-based collaborative
typesetting tool: version control, structured review, and multi-target
publishing for books.

Companion document: [format.md](format.md) — the `.tx` source format.

---

## 1. Design Goals

1. **Git-native.** A book is a git repository. Every txt feature composes
   with plain git; nothing requires the Texture platform. `git clone` +
   `txt build` must always produce the book.
2. **Plain-text source.** Diffable, mergeable, greppable, future-proof.
   No binary project files.
3. **Semantic, not visual.** Authors mark *what things are* (definition,
   exercise, figure); design lives in a separate, versioned design spec.
   The same source builds web, PDF, EPUB, and print-ready files.
4. **Educator-usable.** The CLI serves technical users; the Texture platform
   gives educators a browser experience (edit, comment, review) that
   generates clean git history underneath. Educators never need to learn
   git to contribute — but everything they do *is* git.
5. **Accessibility enforced at build time.** Missing alt text, low-contrast
   design tokens, or non-semantic math are build errors, not lint warnings.
6. **Deterministic builds.** Same commit + same txt version = byte-identical
   output. Required for review integrity and print reproducibility.

## 2. System Overview

```
┌─────────────────────────────────────────────────────────┐
│                   Texture platform (web)                 │
│   visual editor · review UI · roles · notifications      │
│              (thin layer over git + txt)                 │
└───────────────────────────┬─────────────────────────────┘
                            │ git push/pull, txt API
┌───────────────────────────┴─────────────────────────────┐
│                      txt (CLI + library)                 │
│  parse (.tx) → semantic doc model → renderers            │
│  build · check · review tooling · diff · merge drivers   │
└───────────────────────────┬─────────────────────────────┘
                            │
        ┌──────────┬────────┴───────┬──────────────┐
      HTML       PDF (print &     EPUB 3        artifacts
    (web reader)  screen, via      (a11y-        (answer keys,
                  typesetting      conformant)    slides, large
                  engine)                         print)
```

## 3. Repository Model

- **One book = one repo.** (`algebra-1`, `biology`, `us-history-a`)
- **Shared assets** (illustration libraries, design specs, glossaries) are
  their own repos, consumed as pinned dependencies (`txt.lock`).
- Standard layout:

```
algebra-1/
├── book.toml           # metadata: title, subject, grade band, standards,
│                       # edition, language, contributors, license
├── txt.lock            # pinned tool version + dependency versions
├── SUMMARY.tx          # book structure: parts, chapters, order
├── chapters/
│   ├── 01-linear-equations/
│   │   ├── chapter.tx
│   │   ├── exercises.tx
│   │   └── figures/    # SVG masters + alt text sidecars if long-form
├── design/             # design spec ref (usually a dependency, overridable)
├── locale/             # translations: locale/es/chapters/...
└── reviews/            # structured review sign-off records (see §6)
```

- **Branching:** trunk-based. `main` is always buildable (CI-enforced).
  Editions are annotated tags (`ed/2027.1`); print runs pin a tag.
  Errata land on `main` and cherry-pick to edition maintenance branches
  (`ed/2027.x`) when print-relevant.

## 4. The txt CLI

```
txt new <template>        scaffold a book/module from a template
txt build [target]        build html | pdf | print | epub | all
txt check                 structural, accessibility, citation, style checks
txt preview               local live-reload web preview
txt diff <ref> <ref>      semantic diff (moved exercise ≠ deleted+added)
txt fmt                   canonical formatting of .tx source
txt review open|sign      create / cryptographically sign a review record
txt translate status      per-locale coverage & staleness report
txt deps update           update pinned shared-asset dependencies
```

Implementation notes:

- **Language:** Rust for the core (parser, doc model, checkers, HTML/EPUB
  renderers) — single static binary, fast CI, WASM build for in-browser
  preview on the platform.
- **PDF typesetting:** drive **Typst** as the typesetting engine behind the
  PDF/print targets (modern, fast, programmable, open-source). Fallback
  consideration: LaTeX (mature math typesetting, ecosystem). Decision
  gate: math quality benchmark + build determinism test on the pilot book.
  The `.tx` → engine mapping is an internal detail; authors never write
  engine code.
- **Math:** authored in a TeX-compatible math syntax, parsed to a semantic
  tree; rendered as MathML (web/EPUB) and native engine math (PDF).
- **Custom git merge driver + diff driver** registered for `.tx`
  (block-aware, so concurrent edits to different blocks in one file don't
  conflict) — this is a keystone feature for multi-author chapters.

## 5. Build Targets

| Target | Output | Notes |
|--------|--------|-------|
| `html` | Static site (the free digital edition) | Offline-capable, no accounts, no tracking; WCAG 2.2 AA |
| `pdf` | Screen PDF | Hyperlinked, tagged (PDF/UA) |
| `print` | PDF/X for print vendors | Bleed/trim per design spec; deterministic |
| `epub` | EPUB 3 | Accessibility metadata, MathML |
| `large-print` | Print variant | Standard target, not a fork |
| `ancillary` | Teacher edition, answer keys, slide decks | Built from the same source via audience flags (§ format.md) |

## 6. Review & Quality Pipeline

Every change to `main` passes:

1. **CI checks** (`txt check`): parse, structure, required alt text,
   citation presence, link integrity, reading-level advisory, style lint.
2. **Human review gates**, enforced by the platform per path/content type:
   - Subject-matter review (credentialed reviewer for the discipline)
   - Pedagogy review (grade-band appropriateness)
   - Copy edit (house style)
3. **Edition sign-off:** publishing a tagged edition requires signed review
   records (`reviews/`) from the responsible editors — `txt review sign`
   creates a signed, versioned attestation tied to the exact commit. The
   review history ships with the book's source: every published claim is
   traceable to who approved it and when.

Roles and reviewer requirements are defined in
[../community/collaborators.md](../community/collaborators.md).

## 7. The Texture Platform

A web application over hosted git (self-hosted forge or major-forge backend
initially; own storage later if needed):

- **Visual editor** for `.tx` (WYSIWYM — shows semantic blocks, not print
  layout), producing clean commits with real authorship.
- **Suggestion & review UI**: comment on any block; suggestions are branches
  + merge requests under the hood.
- **Role-aware workflow**: the review gates in §6 surface as task queues
  ("3 sections awaiting pedagogy review for grades 6–8").
- **Erratum intake**: public "report an issue" on every page of the web
  reader → triaged issue → fix → visible in the next build. Issue → commit
  → build linkage is public.
- **Identity & credentials**: verified educator profiles (see collaborator
  doc); ORCID supported for academic contributors.
- **No student accounts. No student data.** The platform is for contributors;
  readers need no account. This is a privacy stance and a legal moat
  (COPPA/FERPA stay out of scope).

## 8. Versioning & Editions

- txt itself: semver; `txt.lock` pins the tool per book — a book never
  breaks because the tool moved.
- `.tx` format: versioned in `book.toml` (`format = "1"`); txt ships
  migrations (`txt migrate`).
- Books: continuous `main` + annual(ish) tagged editions per subject cycle;
  print matches a tag exactly; the web edition may run ahead of print
  (labeled "updated since your printing" per section).

## 9. Non-Goals (v1)

- Real-time collaborative cursors (git-mediated collaboration first; CRDT
  layer is a possible v2 platform feature, still committing to git).
- LMS integration, assessment delivery, or any student-facing interactivity
  beyond the readers.
- Arbitrary-layout page design tools — design specs are code, maintained by
  design maintainers.

## 10. Open Questions (tracked as issues once the repo is live)

1. Typst vs LaTeX benchmark for math-heavy print (decision gate, §4).
2. Forge strategy: build on an existing forge's API vs. self-hosted Gitea/
   Forgejo from day one.
3. Figure workflow for non-technical illustrators (SVG round-tripping).
4. Translation tooling: gettext-style segment model vs. whole-file locale
   forks with staleness tracking (current lean: segment-anchored, §format).
5. Signed review records: git commit signatures vs. Sigstore.
