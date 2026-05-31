# Serveur

Le serveur Toboggan lit un fichier TOML et diffuse les slides via WebSocket.

## Démarrage

```bash
toboggan-server presentation.toml
```

## Options

| Option | Défaut | Description |
|---|---|---|
| `--host` | `127.0.0.1` | Adresse d'écoute |
| `--port` | `8080` | Port d'écoute |
| `--public-dir` | — | Dossier de fichiers statiques (images, etc.) |

## Exemple avec images

```bash
toboggan-server \
  --host 0.0.0.0 --port 8081 \
  --public-dir "ma-presentation/public" \
  "ma-presentation/ma-presentation.toml"
```

## Contrôle

Ouvrez `http://<adresse>:<port>` dans votre navigateur. La flèche droite avance, la flèche gauche recule.
