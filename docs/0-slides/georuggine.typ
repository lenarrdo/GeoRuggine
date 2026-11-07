#import "@preview/diatypst:0.9.3": *

#let FONT_SIZE = 8.5pt;

#set text(font: "Fira Sans", size: FONT_SIZE, lang: "it")
#show raw: set text(font: "Fira Mono", size: FONT_SIZE)
#show raw.where(block: true): set text(font: "Fira Mono", size: FONT_SIZE)

#show: slides.with(
  title: "GeoRuggine",
  subtitle: "Sistema di geolocalizzazione per una flotta di veicoli",
  date: "10/11/2026",
  authors: ("Leonardo Miani", "Luca Robustelli"),

  ratio: 16 / 9,
  layout: "medium",
  title-color: rgb("#ef4900").darken(30%),
  theme: "full",
  toc: false,
)


#align(horizon)[
  #include "/sections/1-client.typ"
  #include "/sections/2-server.typ"
]
