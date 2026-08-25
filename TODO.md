# TODOs

## v0.0.1 — Functional editor

Goal: reliably open, edit, and save text files. Comfortable for quick edits to config files, Markdown, scripts, and source files.

### Core terminal

- [ ] Terminal raw mode
- [ ] Keyboard event handling
- [ ] Screen rendering
- [ ] Restore terminal on exit

### Modes

- [ ] View / Edit modes
- [ ] Enter Edit (`i` or dedicated key)
- [ ] Leave Edit (`Esc` → View)
- [ ] Mode indicator in status bar
- [ ] Typing inserts only in Edit
- [ ] `Ctrl` shortcuts identical in both modes

### Editing

- [ ] Cursor movement (arrows)
- [ ] `Ctrl+Arrow` — move by word
- [ ] Text insertion (Edit mode)
- [ ] Text deletion (Backspace / Delete)
- [ ] Scrolling
- [ ] Text selection (`Shift+Arrow`)
- [ ] `Ctrl+A` — select all
- [ ] Copy / cut / paste (`Ctrl+C` / `X` / `V`)
- [ ] Undo / redo (`Ctrl+Z` / `Y`)
- [ ] Search (`Ctrl+F`)

### Files

- [ ] Open file from CLI (`femto path`)
- [ ] Save (`Ctrl+S`)
- [ ] Unsaved-changes warning on quit

### UI

- [ ] Line / column status
- [ ] Shortcut hints (Nano-style, keep small)

### Exit

- [ ] Quit (`Ctrl+Q` or equivalent)
- [ ] Clean shutdown
