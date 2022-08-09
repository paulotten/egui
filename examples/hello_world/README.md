## Linux, MacOS, Windows

```sh
cargo run -p hello_world
```

## Android

WIP, based on https://github.com/rust-windowing/android-ndk-rs#quick-start-hello-world-crate-on-android

Install the Android NDK and SDK as per https://github.com/rust-windowing/android-ndk-rs#1-install-the-android-ndk-and-sdk

Install Android target

```sh
rustup target add aarch64-linux-android
```

Install cargo-apk

```sh
cargo install cargo-apk
```

Run the Android app

```sh
cargo apk run --lib
```
