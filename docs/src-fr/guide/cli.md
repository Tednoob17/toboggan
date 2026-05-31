# Utilisation en ligne de commande

## Convertir des fichiers Markdown en TOML

```bash
toboggan-cli chemin/vers/dossier/ -o presentation.toml
```

## Lire depuis l'entrée standard

```bash
cat slides.md | toboggan-cli -o presentation.toml
```

## Options

| Option | Description |
|---|---|
| `-o, --output` | Fichier de sortie (TOML, JSON, YAML ou HTML) |
| `--public-dir` | Dossier pour les images/ressources |
