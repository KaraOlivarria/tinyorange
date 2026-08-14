# Texture Brand & Style Guide

Version 0.1 (draft) — applies to the organization, the platform, and all
published books unless a book's design spec overrides it.

---

## 1. Naming

| Name | Usage |
|------|-------|
| **Texture** | The organization and the platform. Always capitalized. Never "TEXTURE" or "texture" in prose. |
| **txt** | The tool. Always lowercase, even at the start of a sentence, set in monospace where possible: `txt`. Never "TXT" or "Txt". |
| **the Texture platform** | The hosted collaboration environment (web front-end over txt + git). |
| **Texture Books** | Optional imprint name for print editions. |

Rationale for the wordmark pair: **Texture** is the human, tactile,
paper-and-cloth name; **txt** is its terse machine counterpart — the same
word with everything inessential removed. The brand story is exactly that
relationship: rich books compiled from plain text.

**Tagline:** *Open books. Everyone's knowledge.*
Alternates for testing: *Textbooks, in the open.* / *The source of learning.*

## 2. Logo Direction (brief for a designer)

- **Concept:** a wordmark where the letters T-X-T inside "Te**xt**ure" are
  emphasized (weight or color shift), so the tool's name is literally inside
  the organization's name.
- **Mark:** an abstract "weave" glyph — interlaced horizontal strokes
  suggesting both woven cloth (texture) and lines of text on a page. Must
  reduce cleanly to 16 px (favicon) and single color.
- **Avoid:** book clip-art, mortarboards, apples, owls, lightbulbs.
- Provide: horizontal lockup, stacked lockup, mark-only, single-color
  (black, white), and monochrome-on-photo variants.

## 3. Color

Primary palette (accessible on white; all body-text pairings must meet
WCAG 2.2 AA, 4.5:1):

| Role | Name | Hex | Notes |
|------|------|-----|-------|
| Primary | Ink | `#1A1E23` | Near-black; body text, wordmark |
| Accent | Weft Blue | `#1D5DD8` | Links, primary actions, txt code accent |
| Accent 2 | Loom Green | `#0E7C5B` | Success, "open access" badges |
| Warm | Kraft | `#B4835A` | Print/paper contexts, covers |
| Background | Paper | `#FAF8F4` | Warm off-white page background |
| Neutral | Graphite | `#5B6470` | Secondary text, captions |

Subject-area accent colors (for K-12 book spines/covers) are defined per
series in the book design spec, not here.

## 4. Typography

- **Book body (print & PDF):** a high-quality open-license serif —
  **Source Serif 4** (default) or STIX Two Text for math-heavy books.
- **Book headings & UI:** **Source Sans 3**.
- **Code, txt markup, math source:** **Source Code Pro** / JetBrains Mono.
- **Math rendering:** STIX Two Math.
- Only open-license (OFL) fonts are permitted anywhere in the brand or in
  books — books must be forkable without licensing fonts.

Body text: 10.5–11.5 pt print, 1.45–1.6 line height digital. Measure 60–75
characters. Grade-band adjustments (larger sizes for K-5) are set in the
book design spec.

## 5. Voice & Tone

**The organization** sounds like a great teacher: clear, warm, direct,
confident without hype.

- Plain words over jargon. "Free" not "no-cost solution."
- Active voice. Short sentences. No exclamation inflation.
- We say "educators," not "content creators." We say "students," not "users"
  or "learners" (except in a11y/UX contexts where precision needs it).
- Never disparage commercial publishers by name; the price comparison speaks
  for itself.
- Non-partisan, always. We advocate for open access, not for parties or
  candidates (also a 501(c)(3) requirement).

**The books** follow the Editorial Style Guide (§6) and speak directly to the
student: second person, present tense, concrete examples before abstractions.

## 6. Editorial Style (books & docs)

Baseline: *Chicago Manual of Style* (18th ed.) for books; plain
[developer-docs style] for technical docs. House deviations:

1. Serial comma: always.
2. Numbers: spell out zero through nine in prose; numerals in math/science
   content, data, and grades ("Grade 7").
3. Headings: sentence case everywhere (books, docs, UI).
4. Dates: `August 13, 2026` in prose; `2026-08-13` in metadata and filenames.
5. Units: SI with a space (`9.8 m/s²`); U.S. customary given in parentheses
   where curriculum standards require it.
6. Inclusive language: people-first unless a community prefers
   identity-first; examples and names in books reflect the diversity of the
   students reading them.
7. Citations: every factual claim in a book carries a source reference in the
   `.tx` metadata (rendered or not, it must exist in source).
8. Reading level: each book declares a target band; prose is checked against
   it in CI (advisory, not blocking).

## 7. Accessibility (non-negotiable)

- All digital editions: WCAG 2.2 AA minimum.
- Every figure: required `alt` text in source — `txt` refuses to build
  without it.
- Math: authored semantically, rendered as MathML with speech-ready output.
- Color is never the sole carrier of meaning in figures.
- Print editions: minimum 10 pt body, high-contrast ink/paper pairings;
  large-print builds are a standard build target, not a special project.

## 8. Asset & File Conventions

- Repos: lowercase-kebab (`algebra-1`, `us-history-a`).
- Brand assets live in `assets/brand/` with semantic names
  (`texture-lockup-horizontal.svg`), never `logo_final_v2 (1).svg`.
- Only SVG masters for marks; raster exports are build products.

## 9. Trademark Posture (note for counsel)

Content is CC BY 4.0, but **"Texture," "txt" (as our wordmark), and the
logo are trademarks** of the organization. Forks may use the content freely
but may not present themselves as Texture-published editions. A formal
trademark policy (modeled on Mozilla/Wikimedia policies) is a formation-phase
deliverable — see [../org/formation-checklist.md](../org/formation-checklist.md).
