use std::io::{Error, stdout};

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
        match execute!(
            stdout(),
            EnterAlternateScreen,
            cursor::MoveToColumn(0),
            cursor::MoveToRow(0)
        ) {
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

/// Cursor Position Coordinates
struct CursorPos {
    line: usize,
    byte_offset: usize,
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

    let mut doc_lines: Vec<String> = vec![String::from("")];
    let mut cursor = CursorPos {
        line: 0,
        byte_offset: 0,
    };

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
                code: KeyCode::Esc, ..
            }) => {
                return Ok(());
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char(ch),
                ..
            }) => {
                let byte_len = ch.len_utf8();
                doc_lines[cursor.line].insert(cursor.byte_offset, ch);
                cursor.byte_offset += byte_len;
                redraw(&doc_lines, &cursor)?;
            }
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => {
                let trail = doc_lines[cursor.line].split_off(cursor.byte_offset);
                cursor.line += 1;
                cursor.byte_offset = 0;
                doc_lines.insert(cursor.line, trail);
                redraw(&doc_lines, &cursor)?;
            }
            Event::Key(KeyEvent {
                code: KeyCode::Left | KeyCode::Right,
                ..
            }) => {}
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

/// Renders every document line to the alternate screen.
/// Clears the display and restores the terminal cursor to the logical cursor position.
fn redraw(doc_lines: &[String], cursor_pos: &CursorPos) -> ReturnType<()> {
    let mut output = stdout();
    execute!(
        &mut output,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    )?;

    for (line_number, line) in doc_lines.iter().enumerate() {
        execute!(
            &mut output,
            cursor::MoveTo(0, line_number as u16),
            Print(line)
        )?;
    }

    execute!(
        &mut output,
        cursor::MoveTo(cursor_pos.byte_offset as u16, cursor_pos.line as u16)
    )?;
    Ok(())
}
