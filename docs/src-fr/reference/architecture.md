# Architecture

Toboggan utilise une architecture client-serveur avec WebSocket pour la synchronisation en temps réel.

```
┌──────────────┐   HTTP (page Web)   ┌──────────────┐
│              │ ◄───────────────── │              │
│   Serveur    │                     │   Client     │
│  (Rust/Axon) │   WebSocket (JSON)  │  (Navigateur)│
│              │ ◄───────────────── │              │
└──────────────┘                     └──────────────┘
```

## Flux

1. Le serveur lit le fichier TOML et parse les slides.
2. Le client HTTP sert la page Web.
3. Le client se connecte via WebSocket.
4. Le serveur diffuse les changements de slide à tous les clients connectés.
