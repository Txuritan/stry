# Stry v2

A mini self-hosted Archive Of Our Own, or a story host with tagging.

## Features

- Tagging system
- Search (soon to be added)
- Importer from various sites
  - FanFiction.net (as of now)
- Single user only, made for phones and raspberry pis

## Requirements

- Rust
- SQLite
- NodeJS (when importing)

## Building

- Clone repository
- Either install SQLite or use the bundled version (controlled in `Cargo.toml`)
- Run `cargo build --release`

## License

Stry is licensed under the MIT license I just haven't added it to the repository.
