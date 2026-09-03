# 🪶 femto

![Version](https://img.shields.io/badge/Version-0.1.0-blue.svg)
![Status](https://img.shields.io/badge/Status-ACTIVE%20DEVELOPMENT-Green.svg)

A small terminal text editor in Rust. Batteries included: familiar shortcuts, the usual editing tools, and little else. Built to open a file, change it, and get on with your day.

## What it is / isn't

**It is**
- A small editor that runs in the terminal
- Batteries included: save, copy, cut, paste, undo, search
- Familiar controls, like `Ctrl+S` to save and `Ctrl+C` / `X` / `V` to copy, cut, and paste
- Two modes only: **View** (look around, typing does not insert) and **Edit** (typing inserts text)

**It isn't**
- An IDE
- A plugin platform
- Something you have to study before you can type

## Why

I wanted a terminal editor that works the way I expect:
- save, copy, cut, paste, undo, search on day one
- no extra machinery I never touch
- no new grammar to learn
- light enough to use every day

## Installation

Installation steps will be added once the project is in a running state.

## Development

Enable the repository's pre-commit checks after cloning:

```sh
git config core.hooksPath .githooks
```

Every commit then runs formatting validation, `cargo check`, and all tests.

## Current development status

Raw mode, alternate-screen lifecycle management, basic in-memory text redraw, keyboard input, `Ctrl+Q`, and RAII-based terminal restoration are implemented. See [Current state](docs/current_state.md).

## Roadmap

Two phases:

1. **v0.1.0**: Open, edit, and save files. Cursor, selection, copy/cut/paste, undo, search, and a warning if you quit with unsaved changes.
2. **Later**: Syntax colors, find and replace, more than one file at a time, a config file, and enough polish to use it every day.

See [TODO.md](TODO.md) for the full list.

## License

[MIT](LICENSE)
