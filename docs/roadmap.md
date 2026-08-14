# Texture — Roadmap

Version 0.1 (draft). Dates are placeholders until the founding board sets
them; sequencing is the commitment, not the calendar.

---

## Phase 1 — Foundation (months 0–6)

**Goal: a legal entity, a working toolchain, and the documents in this repo
ratified.**

- [ ] Founding board recruited; incorporation filed
      ([formation checklist](org/formation-checklist.md))
- [ ] Fiscal sponsorship or 1023 filed; bank account; initial fundraising
      (target: 18 months of runway for a 3–5 person core)
- [ ] Domains, trademark filings, brand identity commissioned
      ([brand guide](brand/brand-guide.md))
- [ ] **txt v0.1**: `.tx` parser, `txt build html`, `txt check` with a11y
      gates, `txt fmt`, semantic diff prototype
      ([spec](txt/spec.md))
- [ ] Typst-vs-LaTeX print benchmark decided (spec §10.1)
- [ ] Editorial Board seeded (5 founding members)

**Exit criteria:** a stranger can `git clone` a demo book and build it.

## Phase 2 — Pilot books (months 6–18)

**Goal: prove the editorial pipeline end-to-end on 2–3 K-12 titles.**

Pilot selection criteria: high-enrollment core courses, stable standards,
strong available reviewer pool, and painful incumbent pricing. Candidates:
**Algebra I**, **Biology**, **U.S. History**.

- [ ] Lead Editors appointed; author cohorts recruited
      ([collaborators](community/collaborators.md))
- [ ] txt v1.0: PDF/print + EPUB targets, review attestations, merge driver
- [ ] Texture platform alpha: visual editor, review queues, erratum intake
- [ ] First complete book through all review gates → **Edition 1** tagged
- [ ] Print prototype run; at-cost pricing validated with 1–2 vendors
- [ ] 3–5 pilot classrooms/districts using the digital edition; feedback loop

**Exit criteria:** one published book a district could adopt today.

## Phase 3 — K-12 breadth (years 2–4)

- [ ] Full course sequences: math (6–12), science (MS + Bio/Chem/Physics),
      ELA frameworks, social studies
- [ ] Standards coverage maps per state for adoption committees
- [ ] Elementary (K-5) design language (larger trim, art-heavy design specs)
- [ ] Translation program launch (Spanish first; locale governance stood up)
- [ ] District adoption playbook + support program
- [ ] Sustainability: recurring funder base + print margin covering ops

## Phase 4 — Higher education (years 3–5, overlapping)

- [ ] Target the highest-cost intro courses first: Calculus, Intro Bio,
      Chemistry, Psychology, Economics, Anatomy & Physiology
- [ ] Faculty contributor pipeline (ORCID, departmental partnerships,
      OER-friendly tenure-letter support)
- [ ] Coordination posture with existing OER (OpenStax, LibreTexts, MERLOT):
      complement and interoperate — `.tx` importers for existing OER are a
      bridge, not a rivalry

## Continuous tracks (never "done")

- **Errata & editions:** every book, every year
- **Accessibility:** conformance audits each edition; large-print and
  braille-ready pipelines
- **txt as public infrastructure:** the tool serves any open-book project,
  not only Texture's

## Metrics that matter

Students reached (adoptions × enrollment), dollars saved vs. incumbent
list prices, erratum closure time, review throughput, contributor retention,
locale coverage. Vanity metrics (stars, signups) don't steer decisions.
