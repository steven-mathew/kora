use std::io::{stdout, Write};

use crossterm::event::{Event as InputEvent, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal;
use crossterm::{execute, ErrorKind};

use std::time::Duration;

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
}

impl Terminal {
    pub fn new() -> Result<Self, ErrorKind> {
        let size = terminal::size()?;
        Terminal::enter();
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1,
            },
        })
    }

    pub fn enter() {
        terminal::enable_raw_mode().unwrap();
        execute!(stdout(), terminal::EnterAlternateScreen).unwrap();
    }

    pub fn exit() {
        execute!(stdout(), terminal::LeaveAlternateScreen).unwrap();
        terminal::disable_raw_mode().unwrap();
    }

    pub fn move_to(x: u16, y: u16) {
        execute!(stdout(), crossterm::cursor::MoveTo(x, y)).unwrap();
    }

    pub fn flush() {
        stdout().flush().unwrap();
    }

    pub fn hide_cursor() {
        execute!(stdout(), crossterm::cursor::Hide).unwrap();
    }

    pub fn show_cursor() {
        execute!(stdout(), crossterm::cursor::Show).unwrap();
    }

    pub fn clear() {
        execute!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
    }

    pub fn clear_current_line() {
        execute!(stdout(), terminal::Clear(terminal::ClearType::CurrentLine)).unwrap();
    }

    pub fn size(&self) -> &Size {
        &self.size
    }
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn new() -> Result<Self, ErrorKind> {
        let term = Terminal::new()?;

        Ok(Self {
            quit: false,
            terminal: term,
        })
    }

    pub fn run(&mut self) {
        while !self.quit {
            self.refresh_screen();
            self.process_input();
        }

        Terminal::exit();
    }

    fn read_key(&mut self) -> InputEvent {
        loop {
            if let Ok(true) = crossterm::event::poll(Duration::from_millis(16)) {
                if let Ok(key) = crossterm::event::read() {
                    return key;
                }
            } else {
            }
        }
    }

    fn refresh_screen(&self) {
        // Terminal::clear();
        Terminal::hide_cursor();
        Terminal::move_to(0, 0);

        if self.quit {
            println!("the end.\r");
        } else {
            self.draw_rows();
            Terminal::move_to(0, 0);
        }

        Terminal::show_cursor();
        Terminal::flush();
    }

    fn process_input(&mut self) {
        match self.read_key() {
            InputEvent::Key(key) => self.process_keypress(key),
            InputEvent::Resize(width, height) => {
                self.terminal.size = Size { width, height };

                self.refresh_screen();
            }
            InputEvent::Mouse(_) => (),
        }
    }

    fn process_keypress(&mut self, key: KeyEvent) {
        match key {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL,
            } => self.quit = true,
            _ => (),
        }
    }

    fn draw_intro_message(&self) {
        let mut intro_message = format!("Kora editor -- version {}", VERSION);
        let width = self.terminal.size().width as usize;
        let len = intro_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        intro_message = format!("~{}{}", spaces, intro_message);
        intro_message.truncate(width);
        println!("{}\r", intro_message);
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;

        for row in 1..height {
            Terminal::clear_current_line();
            if row == height / 3 {
                self.draw_intro_message();
            } else {
                println!("~\r");
            }
        }
    }
}

fn main() {
    if let Ok(mut editor) = Editor::new() {
        editor.run();
    }
}
