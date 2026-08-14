// Texture base design spec for secondary math/science books (v0).
// Brand tokens per docs/brand/brand-guide.md. In production this file is
// a versioned dependency (design = "texture-design-secondary-math@…").

#let ink = rgb("1a1e23")
#let weft = rgb("1d5dd8")
#let loom = rgb("0e7c5b")
#let kraft = rgb("b4835a")
#let graphite = rgb("5b6470")

#let serif = ("Source Serif 4", "Libertinus Serif", "New Computer Modern")
#let sans = ("Source Sans 3", "Libertinus Sans", "Liberation Sans")

#let book(title: "", subtitle: none, edition: "", audience: "student", lang: "en", body) = {
  set document(title: title)
  set page(paper: "us-letter", margin: (x: 1.1in, y: 1in), numbering: "1")
  set text(font: serif, size: 11pt, fill: ink, lang: lang)
  set par(justify: true, leading: 0.62em)
  set heading(numbering: none)
  show link: set text(fill: weft)
  show heading.where(level: 1): it => block(above: 0em, below: 1.1em)[
    #text(font: sans, weight: "bold", size: 23pt)[#it.body]
  ]
  show heading.where(level: 2): it => block(above: 1.5em, below: 0.7em)[
    #text(font: sans, weight: "bold", size: 14pt)[#it.body]
  ]
  show heading.where(level: 3): it => block(above: 1.2em, below: 0.5em)[
    #text(font: sans, weight: "bold", size: 12pt)[#it.body]
  ]

  page(numbering: none)[
    #v(2.2in)
    #align(center)[
      #text(font: sans, weight: "bold", size: 34pt)[#title]
      #if subtitle != none {
        v(0.25in)
        text(font: sans, size: 14pt, fill: graphite)[#subtitle]
      }
      #v(0.35in)
      #text(font: sans, size: 11pt, fill: graphite)[#edition]
      #v(0.15in)
      #text(font: sans, size: 10pt, fill: loom, tracking: 0.06em)[
        #upper(audience) EDITION · CC BY 4.0 · TEXTURE
      ]
    ]
  ]
  body
}

#let callout(color, label, body) = block(
  width: 100%,
  inset: (left: 12pt, right: 10pt, top: 9pt, bottom: 9pt),
  stroke: (left: 3pt + color),
  fill: color.lighten(95%),
  breakable: true,
  above: 1em,
  below: 1em,
)[
  #text(font: sans, size: 8pt, weight: "bold", fill: color, tracking: 0.09em)[#upper(label)]
  #v(3pt)
  #body
]

#let tx-objective(body) = block(above: 0.5em, below: 0.5em)[
  #text(font: sans, size: 10pt)[
    #text(fill: loom, weight: "bold")[▸ ] #body
  ]
]
#let tx-definition(label: "Definition", body) = callout(weft, label, body)
#let tx-theorem(label: "Theorem", body) = callout(weft, label, body)
#let tx-note(label: "Note", body) = callout(graphite, label, body)
#let tx-warning(label: "Watch out", body) = callout(kraft, label, body)
#let tx-vocabulary(label: "Vocabulary", body) = callout(loom, label, body)
#let tx-summary(label: "Summary", body) = callout(loom, label, body)
#let tx-activity(label: "Activity", body) = callout(loom, label, body)
#let tx-answer(body) = callout(kraft, "Answer · teacher edition", body)
#let tx-hint(body) = block(above: 0.4em, below: 0.4em, inset: (left: 14pt))[
  #text(font: sans, size: 9.5pt, fill: graphite)[*Hint.* #body]
]

#let tx-example(label: "Example", body) = block(
  width: 100%, above: 1.1em, below: 1.1em,
  inset: (left: 12pt), stroke: (left: 1pt + graphite),
)[
  #text(font: sans, size: 9pt, weight: "bold", fill: ink, tracking: 0.09em)[#upper(label)]
  #v(3pt)
  #body
]
#let tx-solution(body) = block(above: 0.5em, inset: (left: 10pt))[
  #text(font: sans, size: 9pt, weight: "bold", fill: graphite, tracking: 0.09em)[SOLUTION]
  #v(2pt)
  #body
]
#let tx-exercise(label: "Exercise", body) = block(above: 0.9em, below: 0.9em)[
  #text(font: sans, size: 9.5pt, weight: "bold", fill: weft)[#label]
  #h(8pt) #body
]
#let tx-figure(path: "", alt: "", caption: none, credit: none, label: "Figure") = block(
  width: 100%, above: 1.2em, below: 1.2em,
)[
  #align(center)[#image(path, alt: alt, width: 72%)]
  #v(4pt)
  #align(center)[
    #block(width: 85%)[
      #text(font: sans, size: 9pt)[#text(weight: "bold", fill: weft)[#label.] #caption]
      #if credit != none {
        linebreak()
        text(font: sans, size: 8pt, fill: graphite)[#credit]
      }
    ]
  ]
]
