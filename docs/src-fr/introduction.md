# Introduction

**Toboggan** est un système de présentation réparti. Contrairement aux outils de slides classiques (PowerPoint, Keynote, Google Slides), Toboggan sépare l'affichage du contrôle : le serveur fait tourner la présentation, et n'importe quel navigateur Web peut la piloter.

## Pourquoi Toboggan ?

- **Contrôle depuis n'importe quel appareil** — téléphone, tablette, second écran.
- **Plusieurs présentateurs** — avancez les slides chacun de votre côté.
- **Terminaux embarqués** — intégrez des sessions terminal live dans une slide.
- **Pas de cloud** — tout tourne en local sur votre machine.

## Comment ça marche

```
┌─────────────────┐     WebSocket     ┌──────────────┐
│  Serveur        │ ◄──────────────► │  Client Web   │
│  (votre PC)     │                  │  (téléphone)  │
│  slides.toml    │                  └──────────────┘
└─────────────────┘
```

Le serveur lit le fichier de slides et les diffuse via WebSocket. Les clients se connectent et contrôlent la présentation en temps réel.

## Prochaines étapes

- [Installation](installation.md) — installez Toboggan
- [Guide d'utilisation](guide/creating-presentations.md) — créez votre première présentation
