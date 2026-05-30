# Web Client

The web client is the browser-facing UI served by `toboggan-server`.

## Accessing

Start the server, then open:

```text
http://localhost:8080
```

## What it does

- Shows the current slide and presentation state in real time.
- Sends navigation commands back to the server.
- Works as the main presenter view from any modern browser.

## Notes for developers

- The frontend lives in `toboggan-web/`.
- It is built with Vite and TypeScript.
- Dev scripts are available in `package.json`: `dev`, `build`, `preview`, `serve`, `lint`, `format`, and `check`.
