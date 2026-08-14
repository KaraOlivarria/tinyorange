# Texture — Collaborator Roles & Requirements

Version 0.1 (draft). How to contribute to Texture books, what each role
requires, and how work gets reviewed. Governance (who decides) is in
[governance.md](governance.md); licensing terms are in
[../licensing.md](../licensing.md).

---

## 1. Principles

- **Open contribution, gated publication.** Anyone may propose changes;
  what ships passes credentialed review. (Wikipedia's openness, a
  publisher's editorial gate.)
- **Real names, disclosed affiliations** for editorial roles — a textbook's
  credibility depends on who stands behind it.
- **Credit is durable.** Every merged contribution is recorded in git and
  in the book's published contributor listing, per the credit policy (§5).
- **Code of conduct applies everywhere** (platform, reviews, events).
  Enforced by maintainers; appeals to the Editorial Board.

## 2. Roles & Requirements

| Role | What they do | Requirements |
|------|--------------|--------------|
| **Contributor** | Propose fixes, errata, suggestions, exercises | An account and the CLA. No credentials needed — quality is filtered by review. |
| **Author** | Write chapters/sections against the book's outline | Invited or approved by Lead Editors: demonstrated subject expertise (teaching record, degree, or publication history) + writing sample or accepted contributions. |
| **Subject-Matter Reviewer** | Verify correctness | Advanced degree or equivalent professional expertise in the discipline; disclosed affiliations; conflict disclosure (see [COI policy §7](../org/conflict-of-interest-policy.md)). |
| **Pedagogy Reviewer** | Verify grade-band fit, sequencing, learning design | Current or recent classroom experience in the target grade band (teaching credential or documented equivalent) or learning-sciences expertise. |
| **Copy Editor** | House style, clarity, consistency | Professional editing experience; style-guide fluency test. |
| **Accessibility Reviewer** | WCAG conformance, alt-text quality, math speech output | Demonstrated a11y expertise; assistive-technology testing capability. |
| **Illustrator** | Figures per design spec | Portfolio review; SVG workflow; CLA covering artwork. |
| **Translator / Localization Editor** | Locale editions | Native-level fluency + subject familiarity; locale editions get their own review chain. |
| **Lead Editor** (per book) | Owns the book: outline, invitations, merge authority, edition sign-off | Appointed by the Editorial Board; senior subject + teaching credentials; signs published editions. |
| **Maintainer (txt)** | The tool itself | Standard OSS maintainership: sustained quality contributions; elevated by existing maintainers. |

Credential verification is handled privately by the Editorial Board's
staff support (documents are not published; the *verified* status is).
Pseudonymous contribution is welcome at the Contributor level; editorial
roles require verified identity.

## 3. Workflow (how a change ships)

1. **Propose.** Platform suggestion or merge request (a branch, under the
   hood). Errata can start as reader reports from any book page.
2. **Automated checks.** `txt check` in CI: structure, alt text, references,
   style lint, reading level (advisory).
3. **Review gates** (routed by the platform based on what changed):
   - Content changes → Subject-Matter + Pedagogy review
   - Prose-only → Copy edit
   - Figures → Accessibility + design review
4. **Merge.** Lead Editor (or delegated section editor) merges. `main` is
   always publishable.
5. **Edition.** Periodically, the Lead Editor signs an edition tag; signed
   review attestations ship with the source (see
   [../txt/spec.md §6](../txt/spec.md)).

Review SLAs, reviewer honoraria (budgeted per book), and escalation paths
are set per book by the Editorial Board.

## 4. Legal Requirements (all roles)

- **License agreement:** contributions are licensed CC BY 4.0 (content) or
  the code license (tooling) at submission — details and the
  CLA-vs-DCO decision in [../licensing.md](../licensing.md).
- **Originality:** contributors affirm their work is original or properly
  licensed/attributed. Plagiarism or license violation → removal and
  contribution audit.
- **No pay-to-influence.** Payments from third parties to place or slant
  content must be refused and reported (COI policy §7).
- **Minors:** contributors under 18 need guardian consent paperwork
  (platform-enforced); editorial roles are 18+.

## 5. Credit Policy

- Git history is the ground truth of authorship and is never rewritten on
  published branches.
- Published books list: Lead Editor(s), Authors (by chapter), Reviewers,
  Illustrators, Translators, and a link to the full contributor log.
- Roles in the published listing reflect the role performed, decided by the
  Lead Editor per this policy; disputes go to the Editorial Board.
- Employer/funder acknowledgments are permitted in the acknowledgments
  section only — never in content.

## 6. Getting Started (once live)

1. Create a platform account; sign the license agreement.
2. Pick a book with open tasks ("good first errata" queues) or file a
   report from any page of a published book.
3. For editorial roles: apply via the book's Lead Editor with credentials
   and a sample.
