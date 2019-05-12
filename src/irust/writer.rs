use crossterm::{ClearType, Color};

use crate::irust::{IRust, IN, OUT};
use crate::utils::StringTools;

impl IRust {
    pub fn _writeln(&mut self, s: &str) -> std::io::Result<()> {
        self.write_newline()?;
        self._write(s)?;
        Ok(())
    }

    pub fn _write(&mut self, s: &str) -> std::io::Result<()> {
        self.terminal.write(s)?;
        Ok(())
    }

    pub fn write_str_at<P: Into<Option<usize>>, U: Into<Option<usize>>>(
        &mut self,
        s: &str,
        x: P,
        y: U,
    ) -> std::io::Result<()> {
        self.move_cursor_to(x, y)?;
        self.write_str(s)?;
        Ok(())
    }

    fn write_str(&mut self, s: &str) -> std::io::Result<()> {
        self.terminal.clear(ClearType::UntilNewLine)?;
        self.internal_cursor.x += StringTools::chars_count(s);
        self.terminal.write(s)?;
        Ok(())
    }

    pub fn write_in(&mut self) -> std::io::Result<()> {
        self.go_to_cursor()?;
        self.color.set_fg(Color::Yellow)?;
        self.terminal.write(IN)?;
        self.color.reset()?;
        Ok(())
    }

    pub fn write_out(&mut self) -> std::io::Result<()> {
        if !self.output.is_empty() {
            self.go_to_cursor()?;
            self.color.set_fg(Color::Red)?;
            self.terminal.write(OUT)?;
            self.color.reset()?;
            let out = self.output.drain(..).collect::<String>();

            if out.trim().contains('\n') {
                let _ = self.write_newline();
                out.split('\n').for_each(|o| {
                    let _ = self.terminal.write(o);
                    let _ = self.write_newline();
                });
            } else {
                self.terminal.write(out)?;
                self.write_newline()?;
            }
        }

        Ok(())
    }

    pub fn write_insert(&mut self, c: char) -> std::io::Result<()> {
        self.terminal.clear(ClearType::UntilNewLine)?;

        self.terminal.write(c)?;
        self.cursor.save_position()?;
        self.internal_cursor.move_right();

        for character in self.buffer.chars().skip(self.internal_cursor.x) {
            self.terminal.write(character)?;
        }
        self.cursor.reset_position()?;
        Ok(())
    }

    pub fn write_newline(&mut self) -> std::io::Result<()> {
        self.terminal.write('\n')?;
        self.internal_cursor.x = 0;
        self.internal_cursor.y += 1;
        self.go_to_cursor()?;
        Ok(())
    }

    pub fn backspace(&mut self) -> std::io::Result<()> {
        self.terminal.clear(ClearType::UntilNewLine)?;
        self.cursor.save_position()?;

        for character in self.buffer.chars().skip(self.internal_cursor.x) {
            self.terminal.write(character)?;
        }
        self.cursor.reset_position()?;
        Ok(())
    }
}
