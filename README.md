# Jottem

A lean, low friction terminal app for managing markdown notes.

## Installation

### With Cargo via Crates.io

If you have Rust installed, Jottem can be installed directly from [Crates.io](https://crates.io/crates/jottem)

```
cargo install jottem
```

### Build locally

If you wish, you can build Jottem yourself

```
git clone https://github.com/dogue/jottem.git
cd jottem
cargo install --path .
```

### Arch User Repository

Jottem is also available in the AUR with the `jottem-bin` package

```
yay -S jottem-bin
```

## Quick Start

```
# create a note
jottem create my_note

# edit a note
jottem edit my_note

# delete a note
jottem delete my_note

# find a note
jottem find --all
```

You can find a more in-depth explanation of the available commands and their options in [the wiki](https://github.com/dogue/jottem/wiki).

## Key Features

- **Fast:** Jottem uses RocksDB to index notes for quick retrieval and search.
- **Flexible:** Optionally use subdirectories to organize your notes.
- **Tagging:** Categorize notes easily with a simple tagging system.
- **Agnostic:** Jottem uses your `$EDITOR` variable to edit notes in your preferred app.
