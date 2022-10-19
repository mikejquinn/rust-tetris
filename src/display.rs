use crate::util::Color;
use std::io::{self, Write};

const ESC: &'static str = "\x1b";

#[derive(Debug)]
struct Pixel {
    c: char,
    fg_color: Color,
    bg_color: Color,
}

pub struct Display {
    buffer: Vec<Vec<Pixel>>,
}

impl Display {
    pub fn new(width: u32, height: u32) -> Display {
        let mut rows = Vec::with_capacity(height as usize);
        for _ in 0..height {
            let mut row = Vec::with_capacity(width as usize);
            for _ in 0..width {
                row.push(Pixel{ c: ' ', fg_color: Color::Black, bg_color: Color::Black });
            }
            rows.push(row);
        }

        Display {
            buffer: rows
        }
    }

    pub fn render(&mut self) {
        self.clear_screen();
        self.set_cursor_pos(0, 0);

        let mut writer = io::stdout();
        let mut fg_color = Color::Black;
        let mut bg_color = Color::Black;

        let mut y = 0;

        for row in &self.buffer {
            for pixel in row {
                if pixel.fg_color != fg_color {
                    fg_color = pixel.fg_color;
                    self.set_fg_color(pixel.fg_color);
                 }
                if pixel.bg_color != bg_color {
                    bg_color = pixel.bg_color;
                    self.set_bg_color(pixel.bg_color);
                 }

                let bytes = [pixel.c as u8];
                assert!(writer.write_all(&bytes).is_ok());
            }
            y += 1;
            self.set_cursor_pos(0, y);
        }

        assert!(writer.flush().is_ok());
    }

    pub fn set_text(&mut self, text: &'static str, x: u32, y: u32, fg_color: Color, bg_color: Color) {
        let row = &mut self.buffer[y as usize];
        let mut i = 0;

        for c in text.chars() {
            let cell = &mut row[(x + i) as usize];
            cell.c = c;
            cell.fg_color = fg_color;
            cell.bg_color = bg_color;
            i += 1;
        }
    }

    pub fn clear_screen(&self) {
        let mut writer = io::stdout();
        assert!(writer.write_all(self.esc("2J").as_bytes()).is_ok());
        assert!(writer.flush().is_ok());
    }

    pub fn clear_buffer(&mut self) {
        for row in 0..self.buffer.len() {
            for col in 0..self.buffer[row].len() {
                self.buffer[row][col].c = ' ';
                self.buffer[row][col].fg_color = Color::Black;
                self.buffer[row][col].bg_color = Color::Black;
            }
        }
    }

    fn set_cursor_pos(&self, x: u32, y: u32) {
        // Console positions are 1-based
        self.print(&self.esc(&format!("{};{}H", y + 1, x + 1)));
    }

    fn esc(&self, text: &str) -> String { format!("{}[{}", ESC, text) }

    fn print(&self, text: &str) {
        let mut writer = io::stdout();
        assert!(writer.write_all(text.as_bytes()).is_ok());
    }

    fn set_fg_color(&self, color: Color) {
        self.print(&self.esc(&format!("38;5;{}m", self.get_color_code(color))));
    }

    fn set_bg_color(&self, color: Color) {
        self.print(&self.esc(&format!("48;5;{}m", self.get_color_code(color))));
    }

    fn get_color_code(&self, color: Color) -> i32 {
        match color {
            Color::Cyan => 44,
            Color::Purple => 90,
            Color::Green => 2,
            Color::Red => 9,
            Color::Blue => 21,
            Color::Orange => 202,
            Color::Black => 0
        }
    }
}
