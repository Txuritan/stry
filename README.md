<div>
  <p align="center">
    <img align="center" src="./assets/icon.png" alt="stry's home" />
    <h3 align="center">stry</h3>
    <div align="center">
      <strong>A mini self-hosted Archive of Our Own, or a story host with tagging.</strong>
    </div>
    <p align="center">
      <a href="https://github.com/teammycelium/myriad/blob/master/LICENSE">View Demo</a>
      ·
      <a href="https://github.com/teammycelium/myriad/blob/master/LICENSE">Report Bug</a>
      ·
      <a href="https://github.com/teammycelium/myriad/blob/master/LICENSE">Request Feature</a>
    </p>
    <div align="center">
      <img src="https://img.shields.io/badge/made%20with-rust-orange.svg?style=flat-square" alt="Made With Rust" />
      <a href="https://github.com/teammycelium/myriad/blob/master/LICENSE">
        <img src="https://img.shields.io/github/license/teammycelium/myriad.svg?style=flat-square" alt="License" />
      </a>
    </div>
    <div align="center">
      <a href="">
        <img src="https://img.shields.io/gitlab/pipeline/Txuritan/stry2.svg?style=flat-square" alt="Gitlab Build Status" />
      </a>
    </div>
  </p>
</div>

## Table of Contents

  - [About the Project](#about-the-project)
    - [Built With](#built-with)
  - [Getting Started](#getting-started)
    - [Prerequisites](#prerequisites)
    - [Installation](#installation)
  - [Usage](#usage)
  - [Built With](#built-with)
  - [Contributing](#contributing)
  - [License](#license)
  - [Acknowledgements](#acknowledgements)

<img src="./assets/screenshots/stry-home.png" alt="stry's home" />

## Features

  - Tagging system
  - Search
  - Importer from various sites
    - Built upon [story-dl](https://gitlab.com/Txuritan/story-dl)
  - Single user only, made for the phone and raspberry pi
  - Keyboard controls

## Requirements

  - Rust
  - C/C++ compiler

## Building

  - Clone repository
  - Either install SQLite or use the bundled version (controlled in `Cargo.toml`)
  - Run `cargo build --release`, `stry`'s binary will be in `target/release`

## Built With

`stry`, being written in Rust, uses a number of libraries from other developers.
A list can be found in the `Cargo.toml` file but some notable libraries include:

  - The [Tokio](https://github.com/tokio-rs) team's async runtime [Tokio](https://github.com/tokio-rs/tokio) and application level tracing [Tracing](https://github.com/tokio-rs/tracing)
  - [Sean McArthur](https://github.com/seanmonstar)'s async web server [Warp](https://github.com/seanmonstar/warp)
  - [Dirkjan Ochtman](https://github.com/djc)'s compile time template engine [Askama](https://github.com/djc/askama)
  - [John Gallagher](https://github.com/jgallagher)'s SQLite3 bindings [Rusqlite](https://github.com/jgallagher/rusqlite)
  - [Steven Fackler](https://github.com/sfackler/rust-postgres)'s native PostgreSQL driver [Rust-Postgres](https://github.com/sfackler/rust-postgres)

Non managed, but bundled, libraries include:

  - [Jeroen Akkerman](https://github.com/Ionaru)'s markdown editor [EasyMDE](https://github.com/Ionaru/easy-markdown-editor)
  - [Craig Campbell](https://github.com/ccampbell)'s keyboard shortcut handler [Mousetrap](https://github.com/ccampbell/mousetrap)
  - [yairEO](https://github.com/yairEO)'s tag input [Tagify](https://github.com/yairEO/tagify)

## Contributing

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

## Acknowledgements

Thanks to [Archive of Our Own](https://archiveofourown.org/) for being a great inspiration for most of this project, along with being a goal to work towards.
