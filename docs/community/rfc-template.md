# RFC Template

For cross-book standards changes — anything binding more than one book:
the `.tx` block schema, house style, review requirements, credential
criteria, design-system-wide changes. Process defined in
[governance §4](governance.md). Copy this file to `rfcs/NNNN-short-title.md`
in the relevant repo.

---

```markdown
# RFC NNNN: <title>

- **Status:** Draft | Open for comment | Accepted | Rejected | Withdrawn
- **Author(s):**
- **Sponsor:** <Editorial Board member — required to open comment period>
- **Comment period:** <open date> → <close date, min 2 weeks>
- **Decision date / vote record:** <filled by the Board>

## Summary

Two or three sentences. What changes, for whom.

## Motivation

What problem exists today? Cite concrete cases from real books or reviews —
RFCs arguing from hypotheticals are usually premature.

## Design

The change, precisely. For block-schema RFCs: the new/changed block or
attribute, its JSON Schema delta, rendering behavior per build target
(html / pdf / print / epub), and `txt check` rules.

## Impact on existing books

- Which published books are affected, and how (`txt migrate` path if any).
- Backward compatibility: is this additive (minor) or breaking (major)?
  (See [format §8](../txt/format.md).)

## Accessibility impact

Required section, never "N/A" without justification. How does the change
render for screen readers, in large print, in braille-ready output?

## Translation impact

Does this change translation-unit anchoring, add localizable strings, or
invalidate existing locale coverage?

## Alternatives considered

Including "do nothing." Why is this design the right one?

## Open questions

What the comment period should resolve.

## Decision record

Filled in by the Editorial Board on close: outcome, vote, and a short
statement of reasoning. Published with the RFC.
```

---

**Norms:** small RFCs beat large ones; one concern per RFC. An RFC that
would change published student-facing content triggers subject-matter
review before the vote. Rejected RFCs are kept — they document reasoning
for the next person with the same idea.
