# Presenting from Any Device

Toboggan's real-time synchronization lets you control presentations from
any device on the same network — phone, tablet, laptop, or another computer.

## How it works

1. Start the server on your main machine
2. Find your machine's IP address
3. Open any client on any device and point it to the server

## Finding your IP

```bash
# Linux
ip addr show | grep "inet " | grep -v 127.0.0.1

# macOS
ifconfig | grep "inet " | grep -v 127.0.0.1

# Windows (PowerShell)
ipconfig | findstr "IPv4"
```

Typically looks like `192.168.1.42` or `10.0.0.5`.

## Connecting from different devices

### From another computer (same network)

```bash
# Terminal (TUI)
toboggan-tui http://192.168.1.42:8080

# No installation needed — use the web UI
# Just open a browser to:
http://192.168.1.42:8080
```

### From a phone or tablet

No app needed! Just open the browser and navigate to:

```
http://192.168.1.42:8080
```

This works on **iPhone, iPad, Android, and any smartphone**.

### From iOS (native app)

The `toboggan-mobile` crate provides native iOS bindings via UniFFI.
Build the Xcode project and run on your iPhone/iPad.

### From the internet (not just local network)

To present over the internet, you need to expose the server:

**Option A — SSH tunnel (secure, no config)**
```bash
# On a public server
ssh -R 8080:localhost:8080 user@your-server.com

# Then access from anywhere:
http://your-server.com:8080
```

**Option B — ngrok (quick)**

```bash
ngrok http 8080
# Then share the ngrok URL with anyone
```

**Option C — Deploy to a VPS**

```bash
# On your VPS with a public IP
toboggan-server --host 0.0.0.0 --port 8080 talk.toml
```

Then access from anywhere: `http://your-vps-ip:8080`

## Multiple viewers

Toboggan supports **unlimited concurrent viewers**. Everyone connected to
the server sees the same slide in real time. Perfect for:
- Classroom teaching
- Conference presentations
- Remote team meetings
