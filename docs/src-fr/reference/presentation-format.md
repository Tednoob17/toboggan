# Format de présentation

Une présentation Toboggan est un fichier TOML avec une structure spécifique.

## En-tête

```toml
title = "Titre de la présentation"
date = "2025-01-01"
```

## Slides

Chaque slide est un bloc `[[slides]]` :

```toml
[[slides]]
kind = "Standard"  # Cover | Part | Standard

[slides.title]
type = "Text"
text = "Titre de la slide"

[slides.body]
type = "Text"  # ou "Html"
text = "Contenu"
```

## Propriétés optionnelles

| Champ | Type | Description |
|---|---|---|
| `slides.style` | Table | Classes CSS (shadow DOM uniquement) |
| `slides.notes` | Text | Notes présentateur (invisibles pour le public) |
