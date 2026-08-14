# Texture — Founding Pitch Deck

Thirteen slides for foundation program officers and major donors; 20-minute
meeting, 10 slides presented, 3 in reserve. **Slide** = what's on screen
(sparse — the deck is not the document; the
[one-pager](one-pager.md) and this repo are the leave-behind).
**Notes** = what you say. Design: brand palette, Source Serif headlines,
one idea per slide, no stock photos — book spreads and real `.tx` source
are the only imagery.

`$TODO` figures resolve from [budget-model.md](../org/budget-model.md)
once ratified.

---

## 1 · Title

**Slide:** Texture wordmark. *"Open textbooks, written by the world's best
educators, free for every student."*

**Notes:** One sentence of personal grounding — why the founder is doing
this — then straight to the problem. Under 60 seconds.

## 2 · The problem

**Slide:** Three lines, no chart:
*Textbooks cost billions and prices keep climbing. · Errors persist for
years; teachers can't legally adapt their own materials. · The best
educators improve their materials alone, and the work never compounds.*

**Notes:** Pick one concrete anchor story — a district rationing adoptions,
or a student skipping the book. Cost is the door-opener, but the deeper
problem is *wasted expertise*: thousands of educators solve the same
teaching problem every year in private. That framing sets up our solution
as the only durable one.

## 3 · The insight

**Slide:** Two columns.
*Software, 1995: expensive, closed, slowly improving.*
*Software, today: the best infrastructure on earth is free and open.*
Below: *What changed wasn't goodwill. It was tooling.*

**Notes:** Open source won because version control, code review, and
distributed collaboration made mass cooperation practical, and rigorous.
Nobody has built that toolchain for textbooks. OER proved demand; it never
built the infrastructure. That's the founding bet: **the missing piece is
tooling, not content.**

## 4 · What Texture does

**Slide:** Three-part diagram:
**Open books** (CC BY 4.0, free digital, at-cost print) ·
**txt** (git-based collaborative typesetting) ·
**Editorial governance** (credentialed review, independent board).

**Notes:** We are a publisher, a tool-builder, and a standards body — and
each leg needs the others: books without tooling don't stay current;
tooling without editorial rigor doesn't earn adoption; rigor without open
licensing doesn't scale. Competitors have one leg each.

## 5 · The product moment

**Slide:** The landing-page hero, live or as a still: a `.tx` definition
block on the left, `txt build` in the middle, the typeset textbook block on
the right.

**Notes:** This is the demo slide — stay on it as long as questions come.
One plain-text source file; a build step; web, PDF, EPUB, and print out the
other side, with accessibility checked by the machine and content checked
by credentialed reviewers. If the room is technical, show the git history:
who changed this definition, who reviewed it, when.

## 6 · Why open has struggled — and why this time is different

**Slide:** Table, OER status quo vs. Texture:
*static PDFs → living, versioned books · variable quality → signed
editorial review · one-off grants → declining cost per book ·
fragmented sites → one platform, one format.*

**Notes:** Respectful of OpenStax — they proved open textbooks at college
scale; we interoperate, and we start where open is thinnest: K-12 core
curriculum. The structural difference is the repair loop: a teacher reports
an error today, it ships in the next build. No commercial or OER publisher
can say that.

## 7 · Trust: the editorial firewall

**Slide:** Two boxes: *Board of Directors — funds and scopes* ∥
*Editorial Board — decides content.* Caption: *There is no private channel
to change a book. At any price.*

**Notes:** Every funder eventually asks the politics question; answer it
before it's asked. Content follows scholarly consensus, decisions are
published with reasoning, challenges are adjudicated in public. Donors
explicitly cannot buy content changes — that protects the donor too:
no one can accuse a funder of shaping a curriculum through us.

## 8 · Go-to-market

**Slide:** *Algebra I → Biology → U.S. History.* Pilot classrooms →
district adoptions → state alignment maps.

**Notes:** Pilot criteria: high enrollment, stable standards, deep reviewer
pools, painful incumbent pricing. Adoption motion: machine-readable
standards alignment gives committees line-by-line verification instead of
publisher correlation charts; pilot teachers become the reference network.
Free removes the procurement barrier; rigor removes the risk objection.

## 9 · Impact math

**Slide:** One equation, big: *adoptions × enrollment × incumbent price =
dollars returned to schools and families.* Worked once with $TODO
placeholder figures.

**Notes:** Keep claims conservative and sourced; the deck commits to the
*formula* and to publishing real numbers annually. Second-order impact is
the multiplier: every book is adaptable and translatable free, forever —
philanthropy that compounds instead of depreciating.

## 10 · Sustainability

**Slide:** Revenue mix bars, Y1 → Y3: grants shrinking as share, individual
giving growing, modest print margin. Caption: *Our marginal cost per book
falls. Theirs doesn't.*

**Notes:** The infrastructure thesis in one line: the first book costs
~$TODO; each subsequent book reuses the toolchain, design system,
illustration libraries, and reviewer network. We're not asking funders to
subsidize books forever — we're asking them to capitalize infrastructure
that makes books cheap.

## 11 · Roadmap

**Slide:** Four phases from [the roadmap](../roadmap.md): Foundation →
Pilots → K-12 breadth → Higher ed, with exit criteria, not dates.

**Notes:** Emphasize the phase-1 exit test: a stranger can clone the demo
book and build it — infrastructure that works before we scale content.
Milestone-gated, not calendar-gated.

## 12 · Team & governance

**Slide:** Founder(s); founding board seats (filled + open profiles);
Editorial Board structure.

**Notes:** Honest about stage: pre-incorporation, board in formation — and
point at this repository as evidence of execution: complete specs,
governance, and policies before the first dollar. Open board seats are an
invitation; program officers often know the right education leader.

## 13 · The ask

**Slide:** *$TODO founding grants → 18 months → txt v1.0 + Algebra I,
complete, reviewed, printed, and in pilot classrooms.* Contact line.

**Notes:** One number, one deliverable, one timeframe. What we're really
selling: for the price of one district's annual textbook contract, fund the
book *every* district can use — and the machine that makes the next one
cheaper. Then stop talking.

---

## Appendix slides (in reserve)

- **A1 — Budget detail:** per-book unit economics from
  [budget-model.md](../org/budget-model.md).
- **A2 — Legal/licensing:** CC BY (no NC, no SA) rationale; trademark-backed
  quality guarantee; contributor agreement model.
- **A3 — Privacy:** no student accounts, no student data, by architecture.
