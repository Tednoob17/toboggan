# Aperçu développeur

## Structure du projet

```
toboggan/
├── toboggan-core/     # Types partagés (Slide, Config, etc.)
├── toboggan-server/   # Serveur WebSocket + HTTP
├── toboggan-cli/      # Convertisseur CLI
├── toboggan-tui/      # Client terminal (ratatui)
├── toboggan-web/      # Client Web (React + Vite)
├── docs/              # Documentation (mdBook)
└── slides_ex/         # Exemples de présentations
```

## Technologies

- Rust avec Tokio (async) et Axum (HTTP/WebSocket)
- Frontend React + Vite + TypeScript
- Communication via messages JSON sur WebSocket
