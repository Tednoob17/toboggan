# Android client

This page explains how to build and run the Toboggan Android client.

Prerequisites

- Android Studio (latest)
- Android NDK (25+)
- Rust toolchain and Android targets:

```
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android
```

- `mise` helper is used in this repository to orchestrate multi-platform builds (optional).

Build steps (quick)

1. From repository root, build the Rust native library and generated Kotlin bindings:

```
mise build:android
```

This builds the Rust library for Android ABIs, generates UniFFI bindings, and copies `.so` and Kotlin files into the Android app `app/src/main/` layout.

2. Open `toboggan-android/` in Android Studio and build/run the app on an emulator or device.

Connecting to the server

- For the Android emulator, use `10.0.2.2:8080` to reach the host machine's `localhost:8080` (the app defaults to this address).
- For a physical device, update the server URL in `PresentationViewModel.kt` to point to the server host (e.g. `192.168.1.42:8080`).

Notes

- The Android app uses Jetpack Compose and UniFFI-generated Kotlin bindings to the Rust core. If you change Rust APIs, re-run the `mise build:android` step to regenerate bindings.
- If you need to test on multiple ABIs, ensure the `.so` files are present under `app/src/main/jniLibs/`.
