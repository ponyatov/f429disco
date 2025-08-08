// libreadline analog for pure no_std CortexM & embedded Linux

pub struct ReadLine<const N: usize> {
    buffer: [u8; N],
    len: usize,
    cursor: usize,
}

#[cfg(feature="linux")]
use termion::raw::IntoRawMode;

const BACKSPACE: char = '\x08';
const DEL: char = '\x7f';

impl<const N: usize> ReadLine<N> {
    pub fn new() -> Self {
        Self {
            buffer: [0; N],
            len: 0,
            cursor: 0,
        }
    }

    pub fn input(&mut self, ch: char) -> Option<&str> {
        match ch {
            '\r' | '\n' => {
                let result = core::str::from_utf8(&self.buffer[..self.len]).ok();
                self.len = 0;
                self.cursor = 0;
                result
            }
            BACKSPACE | DEL => {
                // Backspace/DEL
                if self.cursor > 0 {
                    self.cursor -= 1;
                    self.len -= 1;
                    for i in self.cursor..self.len {
                        self.buffer[i] = self.buffer[i + 1];
                    }
                }
                None
            }
            c if c.is_ascii_graphic() || c == ' ' => {
                if self.len < N {
                    for i in (self.cursor..self.len).rev() {
                        self.buffer[i + 1] = self.buffer[i];
                    }
                    self.buffer[self.cursor] = c as u8;
                    self.cursor += 1;
                    self.len += 1;
                }
                None
            }
            _ => None,
        }
    }

    pub fn buffer(&self) -> &str {
        core::str::from_utf8(&self.buffer[..self.len]).unwrap_or("")
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    /// set raw mode for console
    pub fn readline(&mut self, prompt: &str) -> Option<&str> {
        print!("{}", prompt);
        
        #[cfg(feature="linux")]
        let raw = std::io::stdout().into_raw_mode().ok();

        loop {
            if let Some(ch) = Self::getchar() {
                if let Some(line) = self.input(ch) {
                    println!();
                    return Some(line);
                }
                // let buffer = self.buffer().to_owned();
                // print!("\r{}{}\x1b[K", prompt, buffer);
            }
        }
    }

    #[cfg(feature = "linux")]
    fn getchar() -> Option<char> {
        use std::io::{stdin, Read};
        let mut buffer = [0; 1];
        stdin().read_exact(&mut buffer).ok()?;
        Some(buffer[0] as char)
    }
}
