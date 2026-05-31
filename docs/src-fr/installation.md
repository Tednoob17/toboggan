# Installation

## Depuis les sources

```bash
git clone https://github.com/Tednoob17/toboggan
cd toboggan
cargo build --release
```

Les binaires se trouvent dans `target/release/` :
- `toboggan-server` — sert les slides
- `toboggan-cli` — convertit les fichiers Markdown en TOML
- `toboggan-tui` — client terminal
- `toboggan-web` — client Web (empaqueté dans le serveur)

## Dépendances

- **Rust** 1.75 ou plus récent (via [rustup](https://rustup.rs/))
- **Node.js** 18+ (uniquement pour le client Web, `toboggan-web/`)

## Client Web

Le client Web est intégré au serveur. Pour le reconstruire :

```bash
cd toboggan-web
npm install
npm run build
```
