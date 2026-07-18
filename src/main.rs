use std::io::{self, Chain, Write, stdout};

use crossterm::cursor::MoveTo;
use crossterm::style::Print;
use crossterm::terminal::ClearType::{All, CurrentLine, Purge};
use crossterm::terminal::{Clear, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{
    event::{KeyCode, read},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use crossterm::{execute, queue};

struct RawGuard;
impl RawGuard {
    fn new() -> io::Result<Self> {
        enable_raw_mode()?;
        execute!(io::stdout(), EnterAlternateScreen)?;
        Ok(RawGuard)
    }
}
impl Drop for RawGuard {
    fn drop(&mut self) {
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        let _ = disable_raw_mode();
    }
}

fn main() -> io::Result<()> {
    let _guard = RawGuard::new()?;
    let mut buf: Vec<String> = vec![String::new()];

    let mut line: usize = 0;

    let mut out = io::stdout();
    while let Ok(event) = read() {
        let Some(event) = event.as_key_press_event() else {
            continue;
        };

        match event.code {
            KeyCode::Backspace => {
                    if !buf[line].is_empty() {
                        buf[line].pop();
                    } else if line > 0 {
                        let removed = buf.remove(line);
                        line-=1;
                        buf[line].push_str(&removed);
                    }
            }
            KeyCode::Char(x) => buf[line].push(x),
            KeyCode::Enter => {
                buf.insert(line+1, String::new());
                line += 1;
            }
            KeyCode::Esc => break,
            _ => {}
        }

        queue!(out, Clear(All), MoveTo(0, 0))?;
        for r in &buf {
            queue!(out, Print(format!("~ {}\r\n", r)))?;
        }
        out.flush()?;
    }
    Ok(())
}
