# Current state

## Features

- Raw terminal mode is enabled before the keyboard event loop.
- `Ctrl+Q` exits the event loop.
- A scoped `TerminalGuard` restores the terminal on normal exit, propagated errors, and panic unwinding.
- A tracked pre-commit hook validates formatting, compilation, and tests when enabled through Git's `core.hooksPath`.

## Caveats

- Cleanup is best-effort because `Drop` cannot return an error.
- Cleanup cannot run when the process aborts or is forcibly terminated.
