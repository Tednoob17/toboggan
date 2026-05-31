# Créer des présentations

Les slides Toboggan se définissent dans un fichier TOML.

## Structure minimale

```toml
title = "Ma Présentation"
date = "2025-01-01"

[[slides]]
kind = "Cover"

[slides.title]
type = "Text"
text = "Ma Présentation"

[slides.body]
type = "Text"
text = "Un sous-titre inspirant"
```

## Types de slides

| `kind` | Usage |
|---|---|
| `Cover` | Page de garde (image pleine page recommandée) |
| `Part` | Interlude / séparateur de section (titre centré) |
| `Standard` | Contenu classique (titre + corps) |

## Contenu riche

Le corps peut être du texte simple ou du HTML :

```toml
[slides.body]
type = "Html"
raw = """
<ul>
  <li>Élément en <strong>gras</strong></li>
  <li style="color: #AD2B43;">Texte coloré</li>
</ul>
"""
```

> **Important** : Toboggan utilise un shadow DOM. Utilisez des styles **inline** — les classes CSS ne fonctionneront pas.
