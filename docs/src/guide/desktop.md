# Desktop App

The desktop app (`toboggan-desktop`) provides a graphical user interface
built with [Iced](https://iced.rs/) and [wgpu](https://wgpu.rs/).

## Running

```bash
# Launch the desktop app
toboggan-desktop

# It will auto-connect to localhost:8080
# Start the server first, then launch the desktop
```

## Workflow

1. Start the server: `toboggan-server my-talk.toml`
2. Launch the desktop: `toboggan-desktop`
3. The desktop connects to the server automatically
4. Use the desktop to navigate slides

## Known behavior

- The desktop app will show "Connection refused" errors if the server
  is not running — this is normal. Start the server first.
- Built-in GPU-accelerated rendering via Vulkan/OpenGL.

## Requirements

- Linux: `libxkbcommon-dev`, `libwayland-dev`, `libegl1-mesa-dev`,
  `libgles2-mesa-dev`
- RAM: ~4 GB during compilation, ~128 MB at runtime
