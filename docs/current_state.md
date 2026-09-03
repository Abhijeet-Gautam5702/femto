# Current state

## Features

- `TerminalGuard` enables raw mode and enters the alternate screen before the keyboard event loop.
- `Ctrl+Q` exits the event loop.
- A scoped `TerminalGuard` leaves the alternate screen and restores raw mode on normal exit, propagated errors, and panic unwinding.
- Printable key presses append to an in-memory buffer and redraw it from the top-left corner.
- A tracked pre-commit hook validates formatting, compilation, and tests when enabled through Git's `core.hooksPath`.

## Caveats

- Cleanup is best-effort because `Drop` cannot return an error.
- If alternate-screen entry fails, raw mode cleanup is attempted before the original error is returned.
- Cleanup cannot run when the process aborts or is forcibly terminated.
- Rendering does not yet clear the viewport or draw an initial frame.
