# Vimeon

Vim-like editor built with [Tauri](https://tauri.studio/en/docs/get-started/intro), [Yew](https://yew.rs/docs/getting-started/introduction), and ❤️

## Installation

```shell
rustup target add wasm32-unknown-unknown
cargo install trunk
cargo install wasm-bindgen-cli
cargo install tauri-cli --version ^1.0.0-beta
```

## Dev Server 

After installing the above, you should be able to run it with

```shell
cargo tauri dev
```

## Building the app

You can do a release build with

```shell
cargo tauri build
```

This should create an installer in src-tauri/target/release/bundle/

Thanks to [stevepryde](https://github.com/stevepryde) for providing an excellent article and [repo](https://github.com/stevepryde/tauri-yew-demo) on how to hook up Tauri and Yew.
