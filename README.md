# jottem

A lightweight terminal app for managing markdown notes.

## What is it?

Jottem is the note manager I wished already existed. It is a simple and lightweight command line tool to manage a collection of markdown files. It focuses on minimizing the friction between the moment you decide you want to write and actually having the file open in your editor.

Jottem keeps your files in a folder known as the `$JOTTEM_ROOT` (see the Configuration section below). It also keeps an index (using RocksDB) of your notes for fast access (retrieving an absolute path rather than walking directories). A search function is planned but is not implemented at this time. Jottem uses your `$EDITOR` variable to open notes.

*Please note that as of the time of this writing, you are looking at a very early version of jottem. There are still features to come and some existing ones may not be fully fleshed out at this point. I will do my best to keep this readme up to date as development progresses.*

## Installation

Clone this repository and install with Cargo like so:

```
git clone https://github.com/dogue/jottem
cd jottem
cargo install --path .
```

## Usage

Jottem's CLI is fairly simple and I've tried to make the help docs you know...helpful. Regardless, let's go over the main functionality.

An important idiom to know is that Jottem allows you to specify a relative path (relative to `$JOTTEM_ROOT`) anywhere that it accepts a note title. This allows you to organize your notes into subdirectories (or not) as you see fit. If you specify a relative path that doesn't exist, Jottem will create it for you. 

For example, if you create a new note and specify the title as `foo/bar/baz`, Jottem will create the folders `foo` and `bar` and create a file called `baz.md` inside of `bar` inside of `foo`.

When attempting to edit a note, you can give the relative path or just the title. If the title doesn't match any known notes, Jottem will ask if you want to create a new one with that title/path. If the title matches exactly one note, that note will be opened in your editor. If the title matches multiple known notes, Jottem will provide you with a list to select one.

### Creating a new note

You can create a new note using the `create` (alias `c`) command like so:

```
jottem c my_note
```

*Jottem does not care what characters you use in your note titles/paths except that it will trim leading and trailing slashes. Otherwise, Linux filesystem rules are the only rules.*

### Editing an existing note

You can edit an existing note using the `edit` (alias `e`) command like so:

```
jottem e my_note
```

This will open the note in your editor if it exists.

*See Usage above for details on how the edit command handles non-existent notes.*

### Finding notes

You can search for notes using the `find` (alias `f`) subcommand like so:

Search by note title/path
```
jottem f -p note_title
```

Search by tags
```
jottem f -t a_tag,another_tag
```

Jottem's tag search is currently very greedy, meaning it will return all notes that match *any* of the provided tags.

### Deleting a note

You can delete a note using the `delete` (alias `d`) command like so:

```
jottem d my_note
```

This command follows the same behavior as the edit command in regards to titles that match multiple notes.

### Future commands

Jottem currently displays a `rebuild` command and a `tag` command. The tag command is not yet implemented. Tags can be added using the `-t` flag when creating a note, but are not yet fully implemented (~~no search yet~~). The rebuild command is not yet implemented but will be used later for rebuilding the index should you decide to move your root folder or otherwise need to resync the index with the note files on disk.

## Configuration

Jottem's configuration is very simple and consists of two environment variables:

`$JOTTEM_ROOT` specifies the root folder under which all of your note files will live. This defaults to `$HOME/.local/share/jottem` if not set.

`$JOTTEM_DB_PATH` specifies where the index database should be stored on disk. This defaults to `$HOME/.cache/jottem` if not set.

## License

[MIT, baby. Always.](LICENSE)

## Contributing

This project was started to satisfy my own desire for a lightweight notes manager with the features *I* wanted. That said, if you're interested in the project, please do consider contributing if you'd like. I welcome your ideas on how to improve the project.
