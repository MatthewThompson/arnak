[![Build](https://img.shields.io/github/actions/workflow/status/MatthewThompson/arnak/ci.yml?branch=main)](https://docs.rs/arnak)
[![crates.io](https://img.shields.io/crates/v/arnak.svg)](https://crates.io/crates/arnak)
[![Docs](https://img.shields.io/badge/docs-online-informational)](https://docs.rs/arnak)
[![License: MIT](https://img.shields.io/badge/license-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Unsafe](https://img.shields.io/badge/unsafe-forbidden-green.svg)](https://github.com/rust-secure-code/safety-dance)

# Arnak
Rust library for [BoardGameGeek XML API](https://boardgamegeek.com/wiki/page/BGG_XML_API2) bindings.

## Usage

```rust
use arnak::BoardGameGeekApi;

// Enter tokio async runtime.
let rt = tokio::runtime::Runtime::new().unwrap();
rt.block_on(async {
    let api = BoardGameGeekApi::new();
    let collection = api.collection().get_owned("bluebearbgg").await;
    match collection {
        Ok(collection) => println!("bluebearbgg owns {} games.", collection.items.len()),
        Err(e) => println!("Error: {e}"),
    }
})
```
