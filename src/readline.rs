// libreadline analog for pure no_std CortexM & embedded Linux

use heapless::String;

pub struct ReadLine<const N: usize> {
    buffer: String<N>,
    cursor: usize,
}

impl<const N: usize> ReadLine<N> {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            cursor: 0,
        }
    }

    pub fn input(&mut self, ch: char) -> Option<String<N>> {
        match ch {
            '\r' | '\n' => {
                let line = self.buffer.clone();
                self.buffer.clear();
                self.cursor = 0;
                Some(line)
            }
            '\x08' | '\x7f' => { // Backspace/DEL
                if self.cursor > 0 {
                    self.buffer.remove(self.cursor - 1);
                    self.cursor -= 1;
                }
                None
            }
            c if c.is_ascii_graphic() || c == ' ' => {
                if self.buffer.len() < N {
                    let _ = self.buffer.insert(self.cursor, c);
                    self.cursor += 1;
                }
                None
            }
            _ => None,
        }
    }

    pub fn buffer(&self) -> &str {
        &self.buffer
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }
}
