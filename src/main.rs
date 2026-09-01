use std::io::{Error, stdout};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::Print,
    terminal,
};

type ReturnType<T> = Result<T, Error>;

/// RAII Guard
struct TerminalGuard;
impl TerminalGuard {
    fn enter_raw_mode() -> ReturnType<Self> {
        terminal::enable_raw_mode()?;
        Ok(Self)
    }
}
impl Drop for TerminalGuard {
    /// Performs best-effort cleanup because `Drop` cannot return an error.
    fn drop(&mut self) {
        let _ = terminal::disable_raw_mode();
    }
}

fn main() -> ReturnType<()> {
    match run() {
        Ok(_) => {}
        Err(e) => {
            execute!(&mut stdout(), Print(format!("Error Occured: {}", e)))?;
        }
    }
    Ok(())
}

fn run() -> ReturnType<()> {
    // A bare `_` would drop the guard immediately; this binding keeps it alive until `run` exits.
    let _terminal_guard = TerminalGuard::enter_raw_mode()?;

    loop {
        let event = event::read()?;
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => {
                return Ok(());
            }
            _ => {
                execute!(
                    &mut stdout(),
                    Print(format!("HELLOW WORLD")),
                    cursor::MoveToNextLine(1),
                    cursor::MoveToColumn(0)
                )?;
            }
        }
    }
}
