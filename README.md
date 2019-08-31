<div>
  <p align="center">
    <img align="center" src="./assets/logo.png" alt="stry's home" />
    <h3 align="center">stry</h3>
    <div align="center">
      <strong>A mini self-hosted Archive Of Our Own, or a story host with tagging.</strong>
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
- [Contributing](#contributing)
- [License](#license)
- [Acknowledgements](#acknowledgements)

<img src="./assets/screenshots/stry-home.png" alt="stry's home" />

## Features

- Tagging system
- Search (soon to be added)
- Importer from various sites
  - FanFiction.net (as of now)
- Single user only, made for phones and raspberry pis

## Requirements

- [Task](https://github.com/go-task/task) task runner
- Rust
- SQLite
- NodeJS (scraper and front-end)

## Building

- Clone repository
- Either install SQLite or use the bundled version (controlled in `site/Cargo.toml`)
- Run `npm install` then `task build`, Stry's binary will be in `target/release` along with the scraper

## Contributing

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details
