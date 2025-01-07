use std::io::{self, Write};
use termion::clear;
use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;

pub struct InputHandler<W: Write> {
    pub stdout: W,
    pub last_ctrl_c: bool,
}

impl<W: Write> InputHandler<W> {
    pub fn clear_screen(&mut self) -> io::Result<()> {
        write!(self.stdout, "{}", clear::All)?;
        write!(self.stdout, "{}", cursor::Goto(1, 1))?;
        self.stdout.flush()?;
        Ok(())
    }

    pub fn write_line(&mut self, text: &str) -> io::Result<()> {
        writeln!(self.stdout, "\r{}", text)?;
        self.stdout.flush()?;
        Ok(())
    }

    pub fn write_str(&mut self, text: &str) -> io::Result<()> {
        write!(self.stdout, "\r{}", text)?;
        self.stdout.flush()?;
        Ok(())
    }

    pub fn read_line(&mut self, prompt: &str) -> io::Result<Option<String>> {
        self.write_str(prompt)?;

        let stdin = io::stdin();
        let mut input_line = String::new();
        let mut cursor_pos = 0;
        let mut keys = stdin.keys();

        loop {
            if let Some(Ok(key)) = keys.next() {
                match key {
                    Key::Char('\n') => {
                        writeln!(self.stdout)?;
                        self.last_ctrl_c = false;
                        break;
                    }
                    Key::Char(c) => {
                        input_line.insert(cursor_pos, c);
                        cursor_pos += 1;
                        write!(self.stdout, "{}", &input_line[cursor_pos - 1..])?;
                        if cursor_pos < input_line.len() {
                            write!(self.stdout, "\x1b[{}D", input_line.len() - cursor_pos)?;
                        }
                        self.last_ctrl_c = false;
                    }
                    Key::Backspace => {
                        if cursor_pos > 0 {
                            cursor_pos -= 1;
                            input_line.remove(cursor_pos);
                            write!(self.stdout, "\x08")?;
                            write!(self.stdout, "{} ", &input_line[cursor_pos..])?;
                            write!(self.stdout, "\x1b[{}D", input_line.len() - cursor_pos + 1)?;
                        }
                        self.last_ctrl_c = false;
                    }
                    Key::Left => {
                        if cursor_pos > 0 {
                            cursor_pos -= 1;
                            write!(self.stdout, "\x1b[D")?;
                        }
                        self.last_ctrl_c = false;
                    }
                    Key::Right => {
                        if cursor_pos < input_line.len() {
                            cursor_pos += 1;
                            write!(self.stdout, "\x1b[C")?;
                        }
                        self.last_ctrl_c = false;
                    }
                    Key::Ctrl('c') => {
                        if self.last_ctrl_c {
                            writeln!(self.stdout, "\r\nOperation cancelled.")?;
                            return Ok(None);
                        } else {
                            self.last_ctrl_c = true;
                        }
                    }
                    _ => {}
                }
                self.stdout.flush()?;
            }
        }

        Ok(Some(input_line))
    }

    pub fn read_line_with_history(
        &mut self,
        prompt: &str,
        history: &[String],
        current_index: &mut Option<usize>,
    ) -> io::Result<Option<String>> {
        self.write_str(prompt)?;

        let stdin = io::stdin();
        let mut input_line = String::new();
        let mut cursor_pos = 0;
        let mut keys = stdin.keys();

        // If we're already at a history index, initialize with that command
        if let Some(index) = current_index {
            if *index < history.len() {
                input_line = history[*index].clone();
                cursor_pos = input_line.len();
                write!(self.stdout, "{}", input_line)?;
            }
        }

        loop {
            if let Some(Ok(key)) = keys.next() {
                match key {
                    Key::Char('\n') => {
                        writeln!(self.stdout)?;
                        break;
                    }
                    Key::Char(c) => {
                        input_line.insert(cursor_pos, c);
                        cursor_pos += 1;
                        write!(self.stdout, "{}", &input_line[cursor_pos - 1..])?;
                        if cursor_pos < input_line.len() {
                            write!(self.stdout, "\x1b[{}D", input_line.len() - cursor_pos)?;
                        }
                        self.last_ctrl_c = false;
                    }
                    Key::Backspace => {
                        if cursor_pos > 0 {
                            cursor_pos -= 1;
                            input_line.remove(cursor_pos);
                            write!(self.stdout, "\x08")?;
                            write!(self.stdout, "{} ", &input_line[cursor_pos..])?;
                            write!(self.stdout, "\x1b[{}D", input_line.len() - cursor_pos + 1)?;
                        }
                        self.last_ctrl_c = false;
                    }
                    Key::Up => {
                        if let Some(index) = current_index {
                            if *index > 0 {
                                *index -= 1;
                            }
                        } else if !history.is_empty() {
                            *current_index = Some(history.len() - 1);
                        }

                        if let Some(index) = current_index {
                            write!(self.stdout, "\r\x1b[K{}", prompt)?;
                            input_line = history[*index].clone();
                            cursor_pos = input_line.len();
                            write!(self.stdout, "{}", input_line)?;
                        }
                        self.last_ctrl_c = false;
                    }
                    Key::Down => {
                        if let Some(index) = current_index {
                            if *index + 1 < history.len() {
                                *index += 1;
                                write!(self.stdout, "\r\x1b[K{}", prompt)?;
                                input_line = history[*index].clone();
                            } else {
                                *current_index = None;
                                write!(self.stdout, "\r\x1b[K{}", prompt)?;
                                input_line = String::new();
                            }
                            cursor_pos = input_line.len();
                            write!(self.stdout, "{}", input_line)?;
                        }
                        self.last_ctrl_c = false;
                    }
                    Key::Left => {
                        if cursor_pos > 0 {
                            cursor_pos -= 1;
                            write!(self.stdout, "\x1b[D")?;
                        }
                        self.last_ctrl_c = false;
                    }
                    Key::Right => {
                        if cursor_pos < input_line.len() {
                            cursor_pos += 1;
                            write!(self.stdout, "\x1b[C")?;
                        }
                        self.last_ctrl_c = false;
                    }
                    Key::Ctrl('c') => {
                        if self.last_ctrl_c {
                            writeln!(self.stdout, "\r\nOperation cancelled.")?;
                            return Ok(None);
                        } else {
                            self.last_ctrl_c = true;
                        }
                    }
                    _ => {}
                }
                self.stdout.flush()?;
            }
        }

        Ok(Some(input_line))
    }
}

impl InputHandler<RawTerminal<io::Stdout>> {
    pub fn new_raw() -> io::Result<Self> {
        Ok(InputHandler {
            stdout: io::stdout().into_raw_mode()?, last_ctrl_c: false,
        })
    }
}
