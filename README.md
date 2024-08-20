[![Build](https://img.shields.io/github/actions/workflow/status/MatthewThompson/arnak/ci.yml?branch=main)](https://docs.rs/arnak)
[![crates.io](https://img.shields.io/crates/v/arnak.svg)](https://crates.io/crates/arnak)
[![Docs](https://img.shields.io/badge/docs-online-informational)](https://docs.rs/arnak)
[![License: MIT](https://img.shields.io/badge/license-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Unsafe](https://img.shields.io/badge/unsafe-forbidden-green.svg)](https://github.com/rust-secure-code/safety-dance)

# Arnak
Rust library for [BoardGameGeek XML API](https://boardgamegeek.com/wiki/page/BGG_XML_API2) bindings.

NOTE: This library is currently in prerelease, not all endpoints have been added and breaking changes could happen often before 1.0.0.

## Example

This example uses [Tokio](https://tokio.rs), so it would also be needed as a dependency:
```toml
[dependencies]
arnak = { version = "0.2.0" }
tokio = { version = "1" }
```

```rust
use arnak::BoardGameGeekApi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = BoardGameGeekApi::new();
    let collection = api.collection().get_owned("bluebearbgg").await;
    match collection {
        Ok(collection) => println!("bluebearbgg owns {} games.", collection.items.len()),
        Err(e) => println!("Error: {e}"),
    }

    Ok(())
}
```

## Endpoints

### Collection

For a given user, return their collection of board games. This does not just mean games owned by the user, but also ones on their wishlist,
previously owned, etc...

### Search

Search for a game, returning everything that matches the search. Also includes a `search_exact` function that will only return exact name matches.

### Hot list

Returns the top 10 currently trending games.

## Formatting

Some rustfmt options used are nightly only. So to format run `cargo +nightly fmt`
