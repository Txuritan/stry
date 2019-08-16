# TODO

- Extract CSS to its own crate for use with Stry, CMC, and Akibisuto projects
- Site scrapping and auto HTML to Markdown
  - Convert Turndown into Rust
- ~~Cut down on dependencies~~
  - ~~Either add crates source to stry~~
  - ~~Or remove and update helpers~~
- Logger for the custom server

- Web interface:
  - ~~Story tile size~~
  - Dark theme
  - ~~Chapter number/title~~
  - ~~Chapter pagination selector~~
  - ~~Page bottom padding~~
  - ~~Darker horizontal brake~~
  - Progress saving
    - Progress bar?
  - Index pagination

- Scraper:
  - Updates
  - GUI?

- Scraper/Sites:
  - Archive of Our Own
  - Wattpad
  - Ficwad

- Database:
  - New shorter IDs (nanoID)
  - Chapter 0
  - Pre/Post authors note
  - Maybe convert to PostgreSQL (async?)
  - Exporter
  - Importer (JSON, MessagePack, SQLite, ZIP/TAR, and/or custom binary)
