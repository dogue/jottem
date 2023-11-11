A lean, low friction terminal app for managing markdown notes.

## Installation

```
# from crates.io
cargo install --locked jottem

# from git
git clone https://github.com/dogue/jottem
cd jottem
cargo install --path .

# from arch user repo
yay -S jottem-bin
```

### Optional Dependencies

- [`git`](https://git-scm.com/)
- [`marksman`](https://github.com/artempyanykh/marksman)

## Quick Start

You can find a more in-depth explanation of the available commands and their options in [the wiki](https://github.com/dogue/jottem/wiki).

```
# create a note (if it doesn't exist)
jottem edit my_note

# edit a note
jottem edit my_note

# delete a note
jottem delete my_note

# find a note
jottem find --all
```

## Key Features

- **Fast:** Jottem uses RocksDB to index notes for quick retrieval and search.
- **Flexible:** Optionally use subdirectories to organize your notes.
- **Tagging:** Categorize notes easily with a simple tagging system.
- **Agnostic:** Jottem uses your `$EDITOR` variable to edit notes in your preferred app.

## License

[MIT](https://github.com/dogue/jottem/blob/main/LICENSE)
