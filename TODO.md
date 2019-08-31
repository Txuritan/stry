# TODO

- Extract CSS to its own crate for use with Stry, CMC, and Akibisuto projects
- ~~Cut down on dependencies~~
  - ~~Either add crates source to stry~~
  - ~~Or remove and update helpers~~
- ~~Logger for the custom server~~

- Web interface:
  - ~~Story tile size~~
  - Dark theme
  - ~~Chapter number/title~~
  - ~~Chapter pagination selector~~
  - ~~Page bottom padding~~
  - ~~Darker horizontal brake~~
  - Progress saving
    - Progress bar?
  - ~~Index pagination~~
  - Keyboard bindings

- Scraper:
  - Updates
  - GUI?
  - Site scrapping and auto HTML to Markdown
    - Convert Turndown into Rust
  - Use a Lua based script instead (loaded by rust)

- Scraper/Sites:
  - Archive of Our Own
  - Wattpad
  - Ficwad

- Database:
  - New shorter IDs (nanoID)
  - Chapter 0
  - Pre/Post authors note
  - Exporter
  - Importer (JSON, MessagePack, SQLite, ZIP/TAR, and/or custom binary)
  - Full text search
