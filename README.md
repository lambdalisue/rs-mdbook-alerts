[![crates.io](https://img.shields.io/crates/v/mdbook-alerts.svg)](https://crates.io/crates/mdbook-alerts)
[![docs.rs](https://docs.rs/mdbook-alerts/badge.svg)](https://docs.rs/mdbook-alerts)
[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Build](https://github.com/lambdalisue/rs-mdbook-alerts/actions/workflows/build.yml/badge.svg)](https://github.com/lambdalisue/rs-mdbook-alerts/actions/workflows/build.yml)
[![Test](https://github.com/lambdalisue/rs-mdbook-alerts/actions/workflows/test.yml/badge.svg)](https://github.com/lambdalisue/rs-mdbook-alerts/actions/workflows/test.yml)
[![Audit](https://github.com/lambdalisue/rs-mdbook-alerts/actions/workflows/audit.yml/badge.svg)](https://github.com/lambdalisue/rs-mdbook-alerts/actions/workflows/audit.yml)

# mdbook-alerts

[mdBook] preprocessor to add [GitHub Flavored Markdown's Alerts](https://docs.github.com/en/get-started/writing-on-github/getting-started-with-writing-and-formatting-on-github/basic-writing-and-formatting-syntax#alerts) to your book like:

```markdown
> [!NOTE]  
> Highlights information that users should take into account, even when skimming.

> [!TIP]
> Optional information to help a user be more successful.

> [!IMPORTANT]  
> Crucial information necessary for users to succeed.

> [!WARNING]  
> Critical content demanding immediate user attention due to potential risks.

> [!CAUTION]
> Negative potential consequences of an action.
```

into

![Rendered example](https://github.com/lambdalisue/rs-mdbook-alerts/blob/main/example/example.png?raw=true)

[mdBook]: https://github.com/rust-lang/mdBook

## Usage

First, install the preprocessor:

```bash
cargo install mdbook-alerts
```

Then, add the preprocessor to your `book.toml`:

```toml
[book]
authors = ["Alisue"]
language = "en"
multilingual = false
src = "src"
title = "mdBook Alerts preprocessor"

# ADD THIS
[preprocessor.alerts]
```

## License

The code follows the MIT license written in [LICENSE](./LICENSE). Contributors
need to agree that any modifications sent to this repository follow the license.
