# ADR 0001 — `.tx` is the language we build; the print engine is a backend we embed

- **Status:** Accepted (founder decision, 2026-08-13)
- **Context docs:** [spec.md](../spec.md), [format.md](../format.md)

## Context

"LaTeX is painful" conflates three layers with different economics:

1. **The authoring language** — backslash soup; content, style, and a
   Turing-complete macro system tangled in one file. Genuinely bad, and
   where our users (educators) actually live.
2. **The compilation model** — multi-pass builds, aux files, errors
   surfacing far from their cause, fragile package interactions. Also bad.
3. **The layout engine** — Knuth-Plass line breaking, math spacing,
   hyphenation, microtypography. *Never the problem.* Decades-hardened;
   reaching parity is person-years of work nobody thanks you for.

The graveyard of LaTeX successors is mostly projects that died rebuilding
layer 3 before shipping their layer-1 innovation. Typst is the exception
that proves the cost: a dedicated, funded team took ~5 years to get layer 3
to "usually as good," and its tail (tagged-PDF accessibility, equation
breaking) is still maturing.

## Decision

1. **Texture's innovation budget goes to layer 1 and 2.** `.tx` is a new
   typesetting *language* — semantic, pedagogy-first, reviewable,
   accessibility-enforcing — and txt is a new *compilation model*
   (single-pass, deterministic, line-accurate errors, git-native). We own
   these end-to-end and they are where we are differentiated. `.tx` gets to
   beat general-purpose systems on ergonomics precisely because it refuses
   generality: a fixed block vocabulary, design as separate versioned code.
2. **Layer 3 is an embedded backend, starting with Typst.** txt compiles
   `.tx` to a semantic document model; a renderer translates that model to
   the engine's input. Authors never see or write engine code, and no
   engine concept may leak into `.tx`.
3. **The backend boundary is a hard interface** (`document model in,
   artifact out`), so the engine is swappable: if the Typst benchmark
   (spec §10.1) fails us, or our accessibility requirements outgrow it,
   alternatives — LaTeX, a browser-based print pipeline, or eventually our
   own engine — slot in without touching `.tx` or any book.

## Consequences

- We ship Algebra I instead of becoming a typesetting research lab.
- We inherit Typst's strengths (Rust, speed, active development) and its
  risks (young project, evolving tagged-PDF story). The interface in (3)
  is the mitigation, and the math-quality benchmark stays a Phase-1 gate.
- TeX-syntax math inside `.tx` is translated by the renderer, never passed
  through raw — the math subset is defined by *our* format spec
  ([format.md §5](../format.md)), not by any engine.
- Revisit trigger: reopen this ADR if (a) the benchmark fails, (b) tagged
  PDF/UA cannot meet our accessibility bar via the backend, or (c) K-5
  art-heavy layout needs exceed what the backend can express.
