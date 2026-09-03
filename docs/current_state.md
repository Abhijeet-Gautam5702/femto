# Current state

## Features

- `TerminalGuard` enables raw mode and enters the alternate screen before the keyboard event loop.
- `Ctrl+Q` and `Esc` exit the event loop.
- A scoped `TerminalGuard` leaves the alternate screen and restores raw mode on normal exit, propagated errors, and panic unwinding.
- Printable key presses insert into an in-memory line buffer; character insertion and Enter redraw the full document and restore the cursor position.
- A tracked pre-commit hook validates formatting, compilation, and tests when enabled through Git's `core.hooksPath`.

## Caveats

- Cleanup is best-effort because `Drop` cannot return an error.
- If alternate-screen entry fails, raw mode cleanup is attempted before the original error is returned.
- Cleanup cannot run when the process aborts or is forcibly terminated.
- An initial frame is not drawn until the first character insertion or Enter key press.
- Cursor rendering currently treats UTF-8 byte offsets as terminal columns; Unicode display-width handling is not implemented.
- Left and Right key events are placeholders and do not yet move the cursor.
