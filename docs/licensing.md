# Texture — Licensing Policy

Version 0.1 (draft). **Have counsel review before the first external
contribution is accepted.**

---

## 1. The commitments

1. Every published book is free to read, copy, adapt, translate, and
   redistribute — including commercially — forever.
2. The tooling (txt, the platform) is free and open-source software.
3. Contributors keep their copyright; they license their work in.
4. The organization can never relicense the commons closed (one-way door,
   encoded in the dissolution clause —
   [bylaws](org/bylaws-outline.md#appendix--irs-required-clauses)).

## 2. License choices

| Asset | License | Rationale |
|-------|---------|-----------|
| Book content (text, exercises, figures) | **CC BY 4.0** | Maximum reuse incl. translation & state adaptation; attribution preserves scholarly credit. BY-SA considered and declined: share-alike friction deters district/teacher remixing and mixing with other OER (OpenStax also chose CC BY). Revisit only with strong evidence. |
| **No NC, ever.** | — | NC would block print partners, teacher-pay-teacher remixes, and integration into commercial platforms that reach students. Access is the point. |
| txt (CLI, parser, renderers) | **Apache-2.0** | Permissive + patent grant; lets districts/vendors embed the toolchain, which spreads the format. |
| Texture platform (server) | **AGPL-3.0** | Keeps hosted derivatives of the collaboration platform open; prevents closed SaaS forks of the community infrastructure. |
| `.tx` format spec, schemas, grammars | **CC BY 4.0** (spec text) + Apache-2.0 (schema files) | Anyone can implement the format. |
| Fonts | OFL only (consumed, not authored) | Books must be forkable without font licensing. |
| Trademarks ("Texture", "txt" wordmark, logo) | **Reserved** — trademark policy | Content is open; identity is not. Forks may not imply Texture endorsement. |

## 3. Contribution licensing: DCO + license grant, not copyright assignment

- Contributors sign a lightweight **Contributor Agreement** at account
  creation: (a) DCO-style certification of originality/right to submit,
  (b) irrevocable license of the contribution under the applicable license
  above (CC BY 4.0 for content; Apache-2.0/AGPL-3.0 for code),
  (c) attribution-name choice for the credit listing.
- **No copyright assignment.** Contributors keep ownership; the commons
  holds a license. This is simpler to sign (educators, minors-with-consent,
  institutional employees) and prevents any future owner from relicensing.
- Institutional contributors (a university, a district) sign an entity
  version covering their staff's contributions where work-for-hire applies.
- Each git commit carries `Signed-off-by` (DCO trailer); the platform adds
  it automatically for platform-mediated edits.

## 4. Third-party material in books

- Only compatible-licensed material may be incorporated: CC BY, CC BY-SA
  (isolated + marked if unavoidable — prefer redrawing/rewriting), CC0,
  public domain.
- **No NC or ND material. No "fair use" reliance** for anything load-bearing
  — fair use doesn't travel with downstream reuse, so it would poison the
  promise in §1. Narrow exceptions (critical commentary quotations) require
  Lead Editor sign-off and `source` metadata flagging.
- Every third-party asset gets a `credit:` + license record in source
  (`txt check` enforces presence).

## 5. Attribution requirements (downstream users)

A conforming reuse credits: title, "Texture" (linked), contributors as
listed, license, and whether changes were made — standard CC BY. The books'
colophon page carries ready-to-copy attribution text so downstream users
get it right by default.

## 6. Print editions

Print partners receive the same CC BY files anyone can download; the
at-cost price commitment is contractual with the partner, not a license
restriction (the license permits anyone to print and sell — our low price,
not legal exclusivity, is what keeps prices honest).
