# Items

A small command-line notebook for titles and optional text. Items supports Linux,
macOS, and Windows. It runs locally with no accounts or services.

## Install

Install the stable Rust toolchain (edition 2024, Rust 1.85 or newer), then from
this directory:

```sh
cargo install --path . --locked
```

## Use

```sh
items add "Books to read" --body "The Left Hand of Darkness"
items list
items list --json
items show 1
items remove 1
items --help
```

Titles must contain non-whitespace text and fit on one line. Bodies can contain
multiple lines. IDs start at 1 and are not reused after deletion. Listing shows
IDs and titles in creation order; `show` also displays the body. Removal is permanent.
For scripts, `list --json` prints a JSON array of objects with `id`, `title`, and
`body`, or `[]` when empty.
Errors go to stderr with exit status 1; usage errors exit with status 2.

## Local data

The only saved data is your notebook, stored as `items.json` in the platform data
directory selected by the `directories` crate:

- Linux: `$XDG_DATA_HOME/items/items.json`, or `~/.local/share/items/items.json`.
- macOS: `~/Library/Application Support/Klyve.items/items.json`.
- Windows: `%APPDATA%\Klyve\items\data\items.json`.

Override the directory when needed:

```sh
items --data-dir ./my-notes add "Weekend plans"
items --data-dir ./my-notes list
```

No configuration is required. The directory is created on the first successful
write. Saves replace the file using a temporary file in the same directory;
unreadable or malformed existing data produces an error. Back up the JSON file
to keep a copy of your notes. Use one writer at a time; concurrent writes are
not coordinated. The app makes no network calls.

## Development

```sh
cargo build --release --locked
cargo test --locked
cargo clippy --all-targets --locked -- -D warnings
cargo fmt --check
cargo run --locked -- list
cargo clean
```

The executable is `target/release/items` (`items.exe` on Windows). Storage unit
tests and command-line integration tests use temporary directories and do not
read or change your personal notebook.

Licensed under the MIT license.
