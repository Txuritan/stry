# TODO

- ~~Extract CSS to its own crate for use with Stry, CMC, and Akibisuto projects~~
- Cut down on dependencies
  - ~~Wait for everyone to update to Tokio v0.2~~
    - ~~The issue is mostly mio and rand~~
- Fill out the README

- Server:
  - ~~Logger for the custom server~~
    - ~~Route logging~~
  - Story and chapter edit API routes
  - Clap arguments/config
  - ~~Maybe use Actix or Warp instead~~
  - CSP header

- Web interface:
  - ~~Story tile size~~
  - ~~Dark theme~~
  - ~~Chapter number/title~~
  - ~~Chapter pagination selector~~
  - ~~Page bottom padding~~
  - ~~Darker horizontal brake~~
  - Progress saving
    - Progress bar?
  - ~~Index pagination~~
  - Keyboard bindings
  - ~~Search~~
    - ~~Save the query either in cookies or query parameter~~
  - Ability to edit stories though the client

- Scraper:
  - Updates
  - GUI?
  - ~~Site scrapping and auto HTML to Markdown~~
    - ~~Convert Turndown into Rust~~
  - Use a script based system instead (loaded by rust)

- Scraper/Sites:
  - ~~Archive of Our Own~~
  - Wattpad
  - Ficwad
  - ~~FanFiction.net~~
    - ~~Fix downloading stories with only one chapter~~

- Database:
  - ~~New shorter IDs (nanoID)~~
  - Chapter 0
  - ~~Pre/Post authors note~~
  - Exporter
  - Importer (JSON, MessagePack, SQLite, ZIP/TAR, and/or custom binary)
  - Full text search
