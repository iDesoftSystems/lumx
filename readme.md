# What's Lumx?

`Lumx` is a lightweight, modular application framework designed to prioritize simplicity and flexibility. It features an easily extensible plug-in architecture, enabling seamless integration with other prominent Rust community projects, such as axum, sea-orm, and many others.

## Getting Started

Add to your `Cargo.toml` dependencies:

```yml
[dependencies]
lumx_core = { git = "https://github.com/iDesoftSystems/lumx.git", branch = "main" }
lumx_axum = { git = "https://github.com/iDesoftSystems/lumx.git", branch = "main" }
```

You can now build your program and easily integrate plugins.

```rust
use lumx_axum::{
    axum::{routing, Router},
    plugin::WebPlugin,
    router::ProgramRoutable,
};
use lumx_core::{program::Program, tokio};

#[tokio::main]
async fn main() {
    Program::new()
        .add_router(router())
        .add_plugin(WebPlugin)
        .run()
        .await
}

fn router() -> Router {
    Router::new().route("/", routing::get(|| async { "Hello, world" }))
}
```

Start your app.

```bash
cargo run
```
