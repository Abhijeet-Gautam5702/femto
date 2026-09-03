# TODOs

## v0.1.0 — Functional editor

Goal: reliably open, edit, and save text files. Comfortable for quick edits to config files, Markdown, scripts, and source files.

### Core terminal

- [x] Terminal raw mode
- [x] Keyboard event handling
- [x] Screen rendering
- [x] Restore terminal on exit

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

- [x] Quit (`Ctrl+Q` or equivalent)
- [x] Clean shutdown

## v0.2.0 — Daily driver

Goal: pleasant enough to use as the default terminal editor. Planned stopping point.

### Editing

- [ ] Syntax highlighting
- [ ] Find and replace
- [ ] Auto-indent
- [ ] Bracket matching
- [ ] Proper Unicode handling

### Files and buffers

- [ ] Multiple files / buffers
- [ ] Fuzzy file opening
- [ ] Crash-safe file writes
- [ ] Reasonable large-file performance

### Config

- [ ] Config file
- [ ] Configurable keybindings

### Polish

- [ ] System clipboard integration
- [ ] Polished terminal UI
