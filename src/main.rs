use anyhow::Ok;
use crossterm::{
    cursor,
    event::{self, read},
    style,
    terminal::{self, EnterAlternateScreen},
    ExecutableCommand, QueueableCommand,
};
use std::io::{stdout, Write};

enum Actions {
    Quit,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    WriteChar(char),
    EnterMode(Mode),
}

enum Mode {
    Normal,
    Insert,
}

struct Editor {
    size: (u16, u16),
    cy: u16,
    cx: u16,
    mode: Mode,
    stdout: std::io::Stdout,
}

impl Drop for Editor {
    fn drop(&mut self) {
        self.stdout.flush().unwrap();

        self.stdout.execute(terminal::LeaveAlternateScreen).unwrap();
        let _ = terminal::disable_raw_mode();
    }
}

impl Editor {
    pub fn new() -> Self {
        let stdout = stdout();
        Editor {
            size: (0, 0),
            cy: 0,
            cx: 0,
            mode: Mode::Normal,
            stdout,
        }
    }

    pub fn draw(&mut self) -> anyhow::Result<()> {
        self.stdout.queue(cursor::MoveTo(self.cx, self.cy))?;
        self.stdout.flush()?;
        Ok(())
    }

    pub fn write_char(&mut self, c: char) -> anyhow::Result<()> {
        self.stdout
            .queue(cursor::MoveTo(self.cx, self.cy))?
            .queue(style::Print(c))?;
        self.cx += 1;
        Ok(())
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        terminal::enable_raw_mode()?;
        self.stdout
            .execute(EnterAlternateScreen)?
            .execute(terminal::Clear(terminal::ClearType::All))?;

        loop {
            self.size = crossterm::terminal::size()?;
            if event::poll(std::time::Duration::from_millis(1000))? {
                // 1.Read input from the keyboard
                // 2.Determines the Action based on the keyboard input and mode
                if let Some(action) = Self::handle_event(&self.mode, read()?)? {
                    match action {
                        Actions::Quit => break,
                        Actions::MoveUp => {
                            self.cy = self.cy.saturating_sub(1u16);
                        }
                        Actions::MoveDown => {
                            if self.cy < self.size.1 {
                                self.cy += 1u16;
                            }
                        }
                        Actions::MoveLeft => {
                            self.cx = self.cx.saturating_sub(1u16);
                        }
                        Actions::MoveRight => {
                            if self.cx < self.size.0 {
                                self.cx += 1u16;
                            }
                        }
                        Actions::EnterMode(Mode::Insert) => self.mode = Mode::Insert,
                        Actions::EnterMode(Mode::Normal) => self.mode = Mode::Normal,
                        Actions::WriteChar(c) => {
                            self.write_char(c)?;
                        }
                    }
                }
            }
            self.draw()?;
        }
        Ok(())
    }

    fn handle_event(mode: &Mode, ev: event::Event) -> anyhow::Result<Option<Actions>> {
        match mode {
            Mode::Normal => Self::handle_normal_event(ev),
            Mode::Insert => Self::handle_insert_event(ev),
        }
    }

    fn handle_normal_event(ev: event::Event) -> anyhow::Result<Option<Actions>> {
        match ev {
            event::Event::Key(event) => match event.code {
                event::KeyCode::Left | event::KeyCode::Char('h') => Ok(Some(Actions::MoveLeft)),
                event::KeyCode::Right | event::KeyCode::Char('l') => Ok(Some(Actions::MoveRight)),
                event::KeyCode::Up | event::KeyCode::Char('k') => Ok(Some(Actions::MoveUp)),
                event::KeyCode::Down | event::KeyCode::Char('j') => Ok(Some(Actions::MoveDown)),
                event::KeyCode::Char('q') => Ok(Some(Actions::Quit)),
                event::KeyCode::Char('i') => Ok(Some(Actions::EnterMode(Mode::Insert))),
                _ => Ok(None),
            },
            _ => Ok(None),
        }
    }

    fn handle_insert_event(ev: event::Event) -> anyhow::Result<Option<Actions>> {
        match ev {
            event::Event::Key(event) => match event.code {
                event::KeyCode::Esc => Ok(Some(Actions::EnterMode(Mode::Normal))),
                event::KeyCode::Char(c) => Ok(Some(Actions::WriteChar(c))),
                _ => Ok(None),
            },
            _ => Ok(None),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let mut editor = Editor::new();
    editor.run()
}
