# trakt-rust

[![ci_badge]][ci] [![discord_badge]][discord] [![docs_badge]][docs] [![cratesio_badge]][cratesio]

A api wrapper in Rust for trakt.tv.

## Installation

Add
```toml
trakt = "0.0.2"
```
to your dependencies

## Example

```rust
use trakt::TraktApi;

fn main() {
    let api = TraktApi::new(
        "YOUR APP ID".to_owned(),
        None,
    );

    dbg!(api.show("fairy-tail").unwrap());
}
```

## Async

trakt-rust also has an async implementation using futures. If you want to use it add this to your Cargo.toml

```toml
[dependencies.trakt]
version = "0.0.2"
default-features = false
features = ["async"]
```

### Example

```rust
use tokio::prelude::Future;
use trakt::asyn::TraktApi;

fn fetch() -> impl Future<Item = (), Error = ()> {
    let api = TraktApi::new(
        "CLIENT_ID".to_owned(),
        None,
    );

    api.show("fairy-tail")
        .map(|data| {
            dbg!(data);
        })
        .map_err(|e| {
            panic!(e);
        })
}

fn main() {
    tokio::run(fetch())
}
```

## License

[MIT][license]

**Powered by**

![trakt]

[license]: https://github.com/Lichthagel/trakt-rust/blob/master/LICENSE
[discord]: https://discordapp.com/invite/0pI32FvBW7M0f6A6
[discord_badge]: https://img.shields.io/discord/148103700743847936.svg?label=Discord&style=flat-square
[docs]: https://docs.rs/trakt
[docs_badge]: https://img.shields.io/badge/docs-online-5023dd.svg?style=flat-square
[cratesio]: https://crates.io/crates/trakt
[cratesio_badge]: https://img.shields.io/crates/v/trakt.svg?style=flat-square
[ci]: https://travis-ci.org/Lichthagel/trakt-rust
[ci_badge]: https://img.shields.io/travis/Lichthagel/trakt-rust.svg?style=flat-square
[trakt]: https://raw.githubusercontent.com/Lichthagel/trakt-rust/master/docs/trakt-wide-red-black.svg