# Web Client

The web client provides a browser-based interface for viewing and
controlling presentations.

## Accessing

Start the server, then open your browser:

```
http://localhost:8080
```

## Features

- Real-time slide display with CSS transitions
- Navigation controls (next/previous/goto)
- Remote control from any device (phone, tablet, laptop)
- Presenter mode with speaker notes

## Presenter view

Access the presenter view at `/presenter`:

```
http://localhost:8080/presenter
```

This shows the current slide, next slide preview, speaker notes, and a timer.

## Kiosk mode

For full-screen public displays:

```
http://localhost:8080/kiosk
```

- Hides navigation UI
- Auto-advances slides
- Perfect for projectors and digital signage
