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

![Rendered example](https://private-user-images.githubusercontent.com/546312/296608023-b5815c46-8d6e-49a9-92e6-7e20f0624bc8.png?jwt=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3MDUyNTU2OTIsIm5iZiI6MTcwNTI1NTM5MiwicGF0aCI6Ii81NDYzMTIvMjk2NjA4MDIzLWI1ODE1YzQ2LThkNmUtNDlhOS05MmU2LTdlMjBmMDYyNGJjOC5wbmc_WC1BbXotQWxnb3JpdGhtPUFXUzQtSE1BQy1TSEEyNTYmWC1BbXotQ3JlZGVudGlhbD1BS0lBVkNPRFlMU0E1M1BRSzRaQSUyRjIwMjQwMTE0JTJGdXMtZWFzdC0xJTJGczMlMkZhd3M0X3JlcXVlc3QmWC1BbXotRGF0ZT0yMDI0MDExNFQxODAzMTJaJlgtQW16LUV4cGlyZXM9MzAwJlgtQW16LVNpZ25hdHVyZT03NmE1ZGJjNmI4MDk1ZTk4OGFkYjA1MjcyYTMzMTVhMGU2MjZlYzM4OGFiYTZiM2M4NWE1MjEwNDFkOTRkOGU3JlgtQW16LVNpZ25lZEhlYWRlcnM9aG9zdCZhY3Rvcl9pZD0wJmtleV9pZD0wJnJlcG9faWQ9MCJ9.DORfgsYzm-vCSTUxCTs6afZfLF0CoAlTgbnYpCRcXZI)

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
