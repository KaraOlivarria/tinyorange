# Texture — Three-Year Budget Model (Skeleton)

Working skeleton for the Form 1023 projections and founding-grant asks.
**All figures are placeholder ranges for board discussion, not
commitments.** The treasurer/accountant owns the real model (spreadsheet in
the private `org-records` repo; this file tracks structure and rationale).

Fiscal year: July 1 – June 30 (aligns with school years and education
funders — [bylaws Art. IX](bylaws-outline.md)).

---

## Expense structure

### Personnel (the dominant line, ~65–75%)

| Role | Y1 | Y2 | Y3 | Notes |
|------|----|----|----|-------|
| Executive Director | 1.0 | 1.0 | 1.0 | May be founder; board sets comp with comparability data (COI policy §5) |
| Editorial Lead | 1.0 | 1.0 | 2.0 | Y3 adds second lead as titles multiply |
| Software Engineers (txt/platform) | 2.0 | 2.5 | 3.0 | Rust core + platform |
| Operations/Development | 0.5 | 1.0 | 1.0 | Grants admin, community ops |
| Book Designer | contract | 0.5 | 1.0 | Design specs are code; needs both skills |

Benefits + payroll taxes: ~25–30% load on salaries.

### Program (per pilot book — the unit economics that matter)

| Line | Range/book | Notes |
|------|-----------|-------|
| Author honoraria/stipends | $40–80k | Cohort of 4–8 authors per title |
| Reviewer honoraria | $15–30k | Subject-matter, pedagogy, copy, a11y passes |
| Illustration | $20–50k | Varies hard by subject (Biology ≫ Algebra) |
| Accessibility audit (external) | $8–15k | Per edition, before sign-off |
| Print prototyping | $5–15k | Proofs, vendor qualification, pilot run |
| Classroom pilots | $10–20k | Teacher stipends, materials, feedback collection |

**Planning figure: ~$100–200k per pilot title, falling with reuse** (shared
illustration libraries, design specs, reviewer pool). This declining unit
cost *is* the infrastructure thesis — track it as a headline metric.

### Operating

Legal (formation, trademarks, agreements — front-loaded in Y1), accounting
& audit, insurance (D&O, general liability), infrastructure (hosting, CI —
modest; static digital editions are cheap to serve), fundraising costs,
travel/convenings.

## Revenue structure

| Source | Y1 | Y2 | Y3 | Notes |
|--------|----|----|----|-------|
| Foundation grants | ~90% | ~75% | ~60% | Founding grants → program grants per subject |
| Individual giving | ~10% | ~15% | ~20% | Openness makes a strong small-donor story |
| Print margin | — | ~5% | ~10% | Modest by design; at-cost commitment caps it |
| Earned (services) | — | — | ~10% | Optional: paid district onboarding/support; board decision, mission-screened |

## Sizing (for the one-pager ask)

- **Y1 total:** roughly $850k–1.1M (team of ~5, txt v1, Algebra I in
  production).
- **Y2:** ~$1.4–1.8M (first title published, two more in production,
  platform alpha).
- **Y3:** ~$2.0–2.6M (K-12 breadth begins, translation program).

18-month founding runway ≈ **$1.3–1.7M** — the number behind the
[one-pager](../outreach/one-pager.md) TODO once ratified.

## Notes for the 1023 narrative

- Print sales are **program revenue** (distributing educational materials
  at cost furthers the exempt purpose), not unrelated business income —
  describe them that way from the start; confirm with the accountant
  (UBIT review, [formation checklist Phase 4](formation-checklist.md)).
- Show the declining cost-per-book curve; the IRS narrative and every
  funder pitch tell the same story: infrastructure makes each book cheaper
  than the last.
