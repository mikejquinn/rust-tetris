use util::Color;
use std::io::{self, Write};

const ESC: &'static str = "\x1b";

#[derive(Debug)]
struct Pixel {
    c: &'static str,
    color: Color,
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
                row.push(Pixel{ c: " ", color: Color::Black });
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
        let mut current_color = Color::Black;

        let mut y = 0;

        for row in &self.buffer {
            for pixel in row {
                if pixel.color != current_color {
                    current_color = pixel.color;
                    self.set_color(pixel.color);
                 }

                assert!(writer.write_all(pixel.c.as_bytes()).is_ok());
            }
            y += 1;
            self.set_cursor_pos(0, y);
        }

        assert!(writer.flush().is_ok());
    }

    pub fn set_pixel(&mut self, text: &'static str, x: u32, y: u32, color: Color) {
        let row = &mut self.buffer[y as usize];
        let cell = &mut row[x as usize];
        cell.c = text;
        cell.color = color;
    }

    pub fn clear_screen(&self) {
        let mut writer = io::stdout();
        assert!(writer.write_all(self.esc("2J").as_bytes()).is_ok());
        assert!(writer.flush().is_ok());
    }

    pub fn clear_buffer(&mut self) {
        for row in 0..self.buffer.len() {
            for col in 0..self.buffer[row].len() {
                self.buffer[row][col].c = " ";
                self.buffer[row][col].color = Color::Black;
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

    fn set_color(&self, color: Color) {
        self.print(&self.esc(self.get_color_code(color)));
    }

    fn get_color_code(&self, color: Color) -> &str {
        match color {
            Color::Cyan => "36m",
            Color::Yellow => "33m",
            Color::Purple => "35m",
            Color::Green => "32m",
            Color::Red => "31m",
            Color::Blue => "34m",
            Color::Orange => "38;5;200m",
            Color::Black => "30m"
        }
    }
}
