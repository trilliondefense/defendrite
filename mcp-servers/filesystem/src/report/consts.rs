pub const REPORT_TEMPLATE: &str = r#"
#import sys: inputs

#let data = json(bytes(inputs.data))

#set page(
  paper: "a4",
  margin: (
    x: 2cm,
    y: 2cm
  )
)

#set text(
  size: 10.5pt,
  font: "Libertinus Serif"
)

#set par(
  justify: true,
  leading: 0.75em
)

#align(center)[
  #text(size: 20pt, weight: "bold")[
    #data.title
  ]

  #v(0.5em)

  #text(size: 10pt, fill: gray)[
    INVESTIGATION REPORT
  ]
]

#line(length: 100%, stroke: 1pt)

#v(1em)

#grid(
  columns: (1fr, 1fr),
  gutter: 1em,

  [
    *Case ID*

    #data.case_id
  ],

  [
    *Status*

    #data.status
  ]
)

#v(1em)

== Executive Summary

#data.summary

== Key Findings

#if data.findings.trim() != "" [
  #data.findings
] else [
  No findings provided.
]

== Recommendations

#if data.recommendations.trim() != "" [
  #data.recommendations
] else [
  No recommendations provided.
]

#v(2em)

#line(length: 100%, stroke: 0.5pt)

#align(center)[
  #text(size: 8pt, fill: gray)[
    Confidential Investigation Report
  ]
]
"#;
