# What's Lumx?

`Lumx` is a lightweight, modular application framework designed to prioritize simplicity and flexibility. It features an easily extensible plug-in architecture, enabling seamless integration with other prominent Rust community projects, such as axum, sea-orm, and many others.

## Getting Started

```bash
cargo add lumx_core
cargo add lumx_axum
```

You can now build your program and easily integrate plugins.

```rust
#[tokio::main]
async fn main() {
    lumx_core::program::Program::new()
        .add_plugin(lumx_axum::WebPlugin)
        .run()
        .await
}
```

Start your app

```bash
cargo run
```
