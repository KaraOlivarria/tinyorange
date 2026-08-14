# Texture — Nonprofit Formation Checklist (U.S. 501(c)(3))

> **This is a working checklist, not legal advice.** Engage a nonprofit
> attorney and an accountant before filing. Many law schools and bar
> associations run free nonprofit-formation clinics.

Assumed path: incorporate as a nonprofit corporation in a U.S. state, then
seek federal 501(c)(3) recognition as a public charity.

---

## Phase 0 — Decisions to make first

- [ ] **Founding board.** Recruit at least 3 unrelated directors (5 is
      better; IRS looks for a majority-independent board). Target mix:
      education leadership, publishing/editorial, software, legal/finance.
- [ ] **State of incorporation.** Usually your home state (simplest, one set
      of filings). Delaware only if counsel advises it.
- [ ] **Name clearance.** Search the state corporate registry, USPTO (TESS)
      for "Texture" in education/publishing classes, and domain/social
      availability. "Texture" is a common word — expect to file as e.g.
      **"Texture Open Textbooks, Inc."** or **"The Texture Foundation"**
      while branding as *Texture*. Register the d/b/a if the state requires it.
- [ ] **Fiscal sponsorship (interim option).** Consider operating under a
      fiscal sponsor (e.g., a Model A/Model C sponsor serving open-source or
      education projects) to accept tax-deductible donations while the
      501(c)(3) application is pending. Decide: sponsor first, or file
      directly.
- [ ] **Domains & handles.** Secure primary domain and org email before any
      public announcement; update README contact.

## Phase 1 — Incorporation (state)

- [ ] Draft & file **Articles of Incorporation** including the three clauses
      the IRS requires (drafted in
      [bylaws-outline.md](bylaws-outline.md#appendix--irs-required-clauses)):
      purpose clause, private-inurement prohibition, dissolution clause.
- [ ] Appoint a **registered agent**.
- [ ] Obtain **EIN** (IRS Form SS-4 — free, online, same day).
- [ ] Hold the **organizational board meeting**: adopt bylaws, elect
      officers (President, Secretary, Treasurer), adopt conflict of interest
      policy, authorize bank account, record minutes.
- [ ] Adopt core policies at or shortly after the first meeting:
  - [ ] [Conflict of interest policy](conflict-of-interest-policy.md) (IRS Form 1023 asks)
  - [ ] Document retention & destruction policy
  - [ ] Whistleblower policy
  - [ ] Gift acceptance policy (matters once print revenue & donations mix)
- [ ] Open a **bank account** in the corporation's name.

## Phase 2 — Federal tax exemption

- [ ] **Form 1023 vs 1023-EZ.** 1023-EZ is only for orgs projecting
      ≤ $50k/yr revenue for 3 years — Texture's model (grants + print sales)
      likely exceeds that fast; plan on the **full Form 1023**.
- [ ] Draft the **narrative of activities**: open textbook creation and free
      distribution is squarely educational under §501(c)(3); print-at-cost
      sales should be described as program-related (advancing the educational
      mission), not as an unrelated business.
- [ ] Prepare 3-year **budget projections** (1023 requirement).
- [ ] File Form 1023 + user fee; expect months of processing. Donations
      received while pending are retroactively deductible if approval is
      granted (communicate this carefully to early donors).
- [ ] On approval: keep the **determination letter** somewhere safe and
      public (funders ask constantly).

## Phase 3 — State & ongoing compliance

- [ ] **State tax exemption** (income/franchise, and sales tax where
      applicable — matters for print production).
- [ ] **Charitable solicitation registration** in the home state; add other
      states as fundraising expands (or use a multistate filing service).
- [ ] Annual calendar: IRS **Form 990**, state annual report, charitable
      registration renewals, board meeting cadence per bylaws.
- [ ] **D&O insurance** and general liability once operations begin.
- [ ] **Public disclosure page**: post Form 1023, determination letter, and
      annual 990s on the website (required on request; posting them is both
      compliant and on-brand for radical openness).

## Phase 4 — Mission-specific legal work

- [ ] **Trademark registration**: "Texture" (word mark, relevant classes),
      the logo, and evaluate "txt" registrability with counsel; publish a
      trademark use policy.
- [ ] **Contributor License Agreement / licensing terms** reviewed by
      counsel — see [../licensing.md](../licensing.md).
- [ ] **COPPA / FERPA / state student-privacy review** before any feature
      that touches students or classroom data (the platform is for educator
      collaboration; keep student data out of scope as long as possible —
      this is a major compliance simplifier).
- [ ] **Print partner agreements** (print-on-demand and offset vendors)
      structured to preserve the at-cost commitment.
- [ ] **UBIT review** with the accountant once print revenue exists.

## Suggested founding budget lines (for the 1023 projection)

Staff/contract (editorial lead, txt engineering), legal & accounting,
infrastructure (hosting, CI), pilot book production (illustration, review
honoraria, accessibility audit), print prototyping, insurance, fundraising.

---

*Formation artifacts (filed articles, minutes, determination letter) should
be kept in a private `org-records` repository; this public checklist tracks
status only.*
