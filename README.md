# Noir Flutter Example - C++ Linking Issue

This is a minimal reproduction example for the C++ runtime linking error when using `noir-rs` with `flutter_rust_bridge` on Android.

## Error

```
dlopen failed: cannot locate symbol "_ZTISt12length_error" referenced by librust_lib_noir_flutter_example.so
```

## Setup

- **noir-rs**: v1.0.0-beta.8-3 with features `["barretenberg", "android-compat"]`
- **flutter_rust_bridge**: 2.11.1
- **Target**: aarch64-linux-android
- **Build system**: Cargokit

## How to reproduce

1. Clone this repository
2. Install Flutter and Rust
3. Run the following commands:

```bash
flutter_rust_bridge_codegen generate
flutter run
```

The app will build successfully but crash at startup with the C++ runtime linking error.

## Project Structure

```
├── lib/
│   ├── main.dart          # Simple Flutter app that calls Noir functions
│   └── src/rust/          # Generated flutter_rust_bridge code
├── rust/
│   ├── Cargo.toml         # Minimal dependencies: noir-rs + flutter_rust_bridge
│   └── src/lib.rs         # Basic Noir functions (setup_srs, prove, verify, etc.)
└── rust_builder/          # Cargokit configuration
```

## Expected behavior

The app should start and allow testing basic Noir functions like SRS setup.

## Actual behavior

App crashes on startup when trying to load the Rust library due to missing C++ runtime symbols.

## Additional notes

- Tried linking with `-lc++` and `-lstdc++` but issue persists
- Added `cxx = "1.0"` dependency but didn't help
