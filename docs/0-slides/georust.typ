#import "@preview/diatypst:0.9.3": *

#set text(font: "Fira Sans", size: 10pt, lang: "it")

#show: slides.with(
  title: "GeoRuggine",
  subtitle: "Sistema di geolocalizzazione per una flotta di veicoli",
  date: "09/11/2026",
  authors: ("Leonardo Miani", "Luca Robustelli"),

  ratio: 16 / 9,
  layout: "medium",
  title-color: rgb("#ef4900").darken(30%),
  toc: false,
)

#include "/sections/1-client.typ"
#include "/sections/2-server.typ"
