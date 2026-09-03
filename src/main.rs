use std::io::{Error, Write, stdout};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::Print,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};

type ReturnType<T> = Result<T, Error>;

/// RAII Guard
struct TerminalGuard;
impl TerminalGuard {
    fn new() -> ReturnType<Self> {
        terminal::enable_raw_mode()?;
        match execute!(stdout(), EnterAlternateScreen) {
            Ok(_) => {
                return Ok(Self);
            }
            Err(e) => {
                let _ = terminal::disable_raw_mode();
                return Err(e);
            }
        }
    }
}
impl Drop for TerminalGuard {
    /// Performs best-effort cleanup because `Drop` cannot return an error.
    fn drop(&mut self) {
        let _ = execute!(stdout(), LeaveAlternateScreen);
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
    let _terminal_guard = TerminalGuard::new()?;

    let mut terminal_buffer = String::from("");

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
            Event::Key(KeyEvent {
                code: KeyCode::Char(ch),
                ..
            }) => {
                terminal_buffer.push(ch);
                redraw(&terminal_buffer)?;
            }
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => {
                terminal_buffer.push('\r');
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

fn redraw(text: &str) -> ReturnType<()> {
    let mut stdout = stdout();
    write!(&mut stdout, "")?;
    execute!(
        stdout,
        cursor::MoveToRow(0),
        cursor::MoveToColumn(0),
        Print(text)
    )?;
    // write!(&mut stdout, "{}", text)?;
    // stdout.flush()?;
    Ok(())
}
