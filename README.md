# trakt-rust

[![discord_badge]][discord] [![docs_badge]][docs] [![cratesio_badge]][cratesio]

A api wrapper in Rust for trakt.tv

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

## License

[MIT][license]

[license]: https://github.com/Lichthagel/trakt-rust/blob/master/LICENSE
[discord]: https://discordapp.com/invite/0pI32FvBW7M0f6A6
[discord_badge]: https://img.shields.io/discord/148103700743847936.svg?label=Discord&style=flat-square
[docs]: https://docs.rs/trakt
[docs_badge]: https://img.shields.io/badge/docs-online-5023dd.svg?style=flat-square
[cratesio]: https://crates.io/crates/trakt
[cratesio_badge]: https://img.shields.io/crates/v/trakt.svg?style=flat-square