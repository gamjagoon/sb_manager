use std::{
    io::{stdout, Write},
    str::Matches,
};

enum Actions {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
}

enum Mode {
    Normal,
    Insert,
}

use anyhow::{anyhow, Ok};
use crossterm::{
    cursor,
    event::read,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand, QueueableCommand,
};

fn main() -> anyhow::Result<()> {
    let mut stdout = stdout();
    let mut mode = Mode::Normal;
    let mut cx = 0;
    let mut cy = 0;

    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    loop {
        stdout.queue(cursor::MoveTo(cx, cy))?;
        stdout.flush()?;

        let ev = 
        match read()? {
            crossterm::event::Event::Key(event) => match event.code {
                crossterm::event::KeyCode::Char('q') => break,
                _ => {}
            },
            _ => {}
        }
    }

    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
