use core::error;
use crossterm::cursor::{MoveTo, Show};
use crossterm::event::KeyCode::{Backspace, Down};
use crossterm::event::KeyEvent;
use crossterm::style::Print;
use crossterm::terminal::ClearType::{All, UntilNewLine};
use crossterm::terminal::{Clear, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{
    event::{KeyCode, read},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use crossterm::{execute, queue};
use std::env;
use std::fmt::Error;
use std::fs::File;
use std::io::{self, BufRead, Chain, Stdout, Write, stdout};
use std::path::{Path, PathBuf};
use thiserror::Error;

use crate::Action::{DeleteBackward, MoveUp, SplitLine};

#[derive(Debug, Error)]
enum Errors {
    #[error("wrong arg count")]
    WrongArgCount,
    #[error("path brokey twin :(")]
    NoPath,
    #[error("io error: {0}")]
    Io(#[from] io::Error),
}

pub struct Position {
    pub row: usize,
    pub col: usize,
}

pub struct Buffer {
    lines: Vec<String>,
}

impl Buffer {
    pub fn new(lines: Vec<String>) -> Self {
        Buffer { lines }
    }

    pub fn get_line(&self, row: usize) -> &str {
        if row > 0 && row < self.line_count() {
            return &self.lines[row];
        }
        ""
    }

    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    pub fn line_len(&self, row: usize) -> usize {
        if row > 0 && row < self.line_count() {
            return self.lines[row].len();
        }
        0
    }

    pub fn insert_char(&mut self, pos: Position, chr: char) {
        todo!()
    }

    pub fn delete_char(&mut self, pos: Position) {
        todo!()
    }

    // Splits line into 2 at position and pushes everything from pos.col -> EOL onto newline at row+1
    pub fn split_line(&mut self, pos: Position) {
        todo!()
    }

    // Appends row onto row-1 and deletes row
    pub fn join_line(&mut self, row: usize) {
        todo!()
    }
}

pub struct Cursor {
    pos: Position,
    des_col: usize,
}

struct Term {
    out: Stdout,
}

impl Term {
    fn new() -> io::Result<Self> {
        enable_raw_mode()?;
        let mut out = io::stdout();
        execute!(out, EnterAlternateScreen)?;
        Ok(Term { out })
    }
}

impl Drop for Term {
    fn drop(&mut self) {
        let _ = execute!(&mut self.out, LeaveAlternateScreen);
        let _ = disable_raw_mode();
    }
}

enum Action {
    InsertChar(char),
    DeleteBackward,
    DeleteForward,
    SplitLine,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Quit,
}

fn map_key(key: Option<KeyEvent>) -> Option<Action> {
    let Some(key) = key else {
        return None;
    };

    match key.code {
        KeyCode::Backspace => Some(Action::DeleteBackward),
        KeyCode::Char(x) => Some(Action::InsertChar(x)),
        KeyCode::Enter => Some(Action::SplitLine),
        KeyCode::Up => Some(Action::MoveUp),
        KeyCode::Down => Some(Action::MoveDown),
        KeyCode::Right => Some(Action::MoveRight),
        KeyCode::Left => Some(Action::MoveLeft),
        KeyCode::Esc => Some(Action::Quit),
        _ => None,
    }
}

pub struct Editor {
    cursor: Cursor,
    buffer: Buffer,
    term: Term,
    f_path: PathBuf,
}

impl Editor {
    pub fn new(f_path: &Path) -> Self {
        let lines = Self::file_read(f_path).unwrap_or(vec![String::new()]);
        Editor {
            cursor: Cursor {
                pos: Position { row: 0, col: 0 },
                des_col: 0,
            },
            buffer: Buffer::new(lines),
            term: Term::new().unwrap(),
            f_path: PathBuf::from(f_path),
        }
    }

    pub fn file_read(f_path: &Path) -> io::Result<Vec<String>> {
        let file = File::open(f_path)?;
        let reader = io::BufReader::new(file);
        reader.lines().collect()
    }

    pub fn display(&mut self) -> io::Result<()> {
        let rows = self.buffer.line_count();
        queue!(self.term.out, MoveTo(0, 0)).unwrap();
        for i in 0..rows {
            queue!(
                self.term.out,
                MoveTo(0, i as u16),
                Print("~ "),
                Clear(UntilNewLine)
            )
            .unwrap()
        }
        queue!(
            self.term.out,
            MoveTo((self.cursor.pos.col + 2) as u16, self.cursor.pos.row as u16)
        )?;
        self.term.out.flush()
    }

    fn apply(&mut self, action: Action) {
        let row = self.cursor.pos.row;
        let col = self.cursor.pos.col;

        match action {
            Action::DeleteBackward => {
                if col > 0 {
                    self.buffer.delete_char(self.cursor.pos);
                } else if col 
            },
            Action::DeleteForward =>  
        }
    }

    pub fn run_editor(&mut self) -> io::Result<()> {
        self.display()?;

        while let Ok(event) = read() {
            let Some(event) = map_key(event.as_key_press_event()) else {
                continue;
            };

            self.apply(event);
            self.display()?;
        }

        Ok(())
    }
}

/// OLD CODE BELOW! /////////////////////////////////
fn parse_args() -> Result<PathBuf, Errors> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(Errors::WrongArgCount);
    }

    let file_path = PathBuf::from(&args[1]);

    if !file_path.is_file() {
        return Err(Errors::NoPath);
    }

    Ok(file_path)
}

fn main() -> Result<(), Errors> {
    let f_path = parse_args()?;
    let mut editor: Editor = Editor::new(&f_path);

    editor.run_editor()?;

    Ok(())
}

fn run_editor(mut buf: Vec<String>) -> io::Result<()> {
    let _guard = RawGuard::new()?;

    let mut line: usize = 0;
    let mut col: usize = 0;

    let mut out = io::stdout();
    disp(&buf, &mut out, line, col)?;

    while let Ok(event) = read() {
        let Some(event) = event.as_key_press_event() else {
            continue;
        };

        match event.code {
            KeyCode::Backspace => {
                if !buf[line].is_empty() {
                    buf[line].remove(col);
                } else if line > 0 {
                    let removed = buf.remove(line);
                    line -= 1;
                    buf[line].push_str(&removed);
                }
            }
            KeyCode::Char(x) => {
                buf[line].insert(col, x);
                col += 1;
            }
            KeyCode::Enter => {
                buf.insert(line + 1, String::new());
                line += 1;
            }
            KeyCode::Down => {
                if line + 1 < buf.len() {
                    line += 1;
                    col = 0;
                }
            }
            KeyCode::Up => {
                if line > 0 {
                    line -= 1;
                    col = 0;
                }
            }
            KeyCode::Right => {
                if col == 0 || col + 1 < buf[line].len() {
                    col += 1;
                }
            }
            KeyCode::Left => {
                if col > 0 {
                    col -= 1;
                }
            }
            KeyCode::Esc => break,
            _ => {}
        }
        disp(&buf, &mut out, line, col)?;
    }
    Ok(())
}
