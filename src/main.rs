extern crate libc;
extern crate rand;

mod util;
mod display;
mod terminal;

use std::fmt::{Formatter, Result};
use display::Display;
use util::*;

const BOARD_WIDTH: u32 = 10;
const BOARD_HEIGHT: u32 = 20;
const HIDDEN_ROWS: u32 = 2;

enum Key {
    Up,
    Down,
    Left,
    Right,
    Space,
    CtrlC,
    Char(char),
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

struct Board {
    cells: [[Option<Color>; BOARD_WIDTH as usize]; BOARD_HEIGHT as usize],
}

impl Board {
    pub fn render(&self, display: &mut Display) {
        for y in HIDDEN_ROWS..BOARD_HEIGHT {
            display.set_pixel("|", 0, y, Color::Red, Color::Black);
            display.set_pixel("|", BOARD_WIDTH * 2 + 1, y, Color::Red, Color::Black);
        }
        for x in 0..(BOARD_WIDTH * 2 + 1) {
            display.set_pixel("-", x, BOARD_HEIGHT, Color::Red, Color::Black);
        }
        for row in 0..BOARD_HEIGHT {
            for col in 0..BOARD_WIDTH {
                match self.cells[row as usize][col as usize] {
                    Some(color) => {
                        let c = 1 + (col * 2);
                        display.set_pixel(" ", c, row, color, color);
                        display.set_pixel(" ", c + 1, row, color, color);
                    },
                    None => ()
                }
            }
        }
    }

    pub fn lock_piece(&mut self, piece: &Piece, origin: Point) {
        for row in 0..piece.shape.len() {
            for col in 0..piece.shape[row].len() {
                if piece.shape[row][col] == 1 {
                    let x = origin.x + (col as i32);
                    let y = origin.y + (row as i32);
                    self.cells[y as usize][x as usize] = Some(piece.color);
                }
            }
        }
    }

    pub fn collision_test(&self, piece: &Piece, origin: Point) -> bool {
        for row in 0..piece.shape.len() {
            for col in 0..piece.shape[row].len() {
                if piece.shape[row][col] == 1 {
                    let x = origin.x + (col as i32);
                    let y = origin.y + (row as i32);
                    if x < 0 || x >= (BOARD_WIDTH as i32) || y < 0 || y >= (BOARD_HEIGHT as i32) ||
                       self.cells[y as usize][x as usize] != None {
                        return true;
                    }
                }
            }
        }

        false
    }
}

struct Piece {
    name: &'static str,
    color: Color,
    shape: Vec<Vec<u8>>,
}

impl Clone for Piece {
    fn clone(&self) -> Piece {
        let mut p = Piece{
            name: self.name,
            color: self.color,
            shape: Vec::with_capacity(self.shape.len())
        };
        for row in &self.shape {
            p.shape.push(row.clone());
        }
        p
    }
}

impl Piece {
    pub fn new_o() -> Piece {
        Piece{
            name: "I",
            color: Color::Cyan,
            shape: vec![vec![1, 1],
                        vec![1, 1]]
        }
    }

    pub fn new_l() -> Piece {
        Piece{
            name: "L",
            color: Color::Orange,
            shape: vec![vec![0, 0, 1],
                        vec![1, 1, 1],
                        vec![0, 0, 0]]
        }
    }

    pub fn new_j() -> Piece {
        Piece{
            name: "J",
            color: Color::Blue,
            shape: vec![vec![1, 0, 0],
                        vec![1, 1, 1],
                        vec![0, 0, 0]]
        }
    }

    pub fn new_t() -> Piece {
        Piece{
            name: "T",
            color: Color::Purple,
            shape: vec![vec![0, 1, 0],
                        vec![1, 1, 1],
                        vec![0, 0, 0]]
        }
    }

    pub fn new_s() -> Piece {
        Piece{
            name: "S",
            color: Color::Green,
            shape: vec![vec![0, 1, 1],
                        vec![1, 1, 0],
                        vec![0, 0, 0]]
        }
    }

    pub fn new_z() -> Piece {
        Piece{
            name: "Z",
            color: Color::Red,
            shape: vec![vec![1, 1, 0],
                        vec![0, 1, 1],
                        vec![0, 0, 0]]
        }
    }

    pub fn new_i() -> Piece {
        Piece{
            name: "I",
            color: Color::Cyan,
            shape: vec![vec![0, 0, 0, 0],
                        vec![1, 1, 1, 1],
                        vec![0, 0, 0, 0],
                        vec![0, 0, 0, 0]]
        }
    }

    fn rotate(&mut self, direction: Direction) {
        let size = self.shape.len();

        for row in 0..size/2 {
            for col in row..(size - row - 1) {
                let t = self.shape[row][col];

                match direction {
                    Direction::Left => {
                        self.shape[row][col] = self.shape[col][size - row - 1];
                        self.shape[col][size - row - 1] = self.shape[size - row - 1][size - col - 1];
                        self.shape[size - row - 1][size - col - 1] = self.shape[size - col - 1][row];
                        self.shape[size - col - 1][row] = t;
                    },
                    Direction::Right => {
                        self.shape[row][col] = self.shape[size - col - 1][row];
                        self.shape[size - col - 1][row] = self.shape[size - row - 1][size - col - 1];
                        self.shape[size - row - 1][size - col - 1] = self.shape[col][size - row - 1];
                        self.shape[col][size - row - 1] = t;
                    }
                }
            }
        }
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Piece: {}", self.name)
    }
}

struct PieceBag {
    pieces: Vec<Piece>
}

impl PieceBag {
    fn new() -> PieceBag {
        let mut p = PieceBag{
            pieces: Vec::new()
        };
        p.fill_bag();
        p
    }

    fn pop(&mut self) -> Piece {
        let piece = self.pieces.remove(0);
        if self.pieces.is_empty() {
            self.fill_bag();
        }
        piece
    }

    fn peek(&self) -> &Piece {
        &self.pieces[0]
    }

    fn fill_bag(&mut self) {
        use rand::Rng;

        let mut pieces: Vec<Piece> = vec![
            Piece::new_o(),
            Piece::new_l(),
            Piece::new_j(),
            Piece::new_t(),
            Piece::new_s(),
            Piece::new_z(),
            Piece::new_i()
        ];

        let mut rng = rand::thread_rng();
        while !pieces.is_empty() {
            let i = rng.gen::<usize>() % pieces.len();
            self.pieces.push(pieces.swap_remove(i));
        }
    }
}

struct Game {
    board: Board,
    piece_bag: PieceBag,
    piece: Piece,
    piece_position: Point,
}

impl Game {
    fn new() -> Game {
        let mut piece_bag = PieceBag::new();
        let piece = piece_bag.pop();

        let mut game = Game {
            board: Board{
                cells: [[None; BOARD_WIDTH as usize]; BOARD_HEIGHT as usize]
            },
            piece_bag: piece_bag,
            piece: piece,
            piece_position: Point{ x: 0, y: 0 }
        };

        game.place_new_piece();
        game
    }

    fn find_dropped_origin(&self) -> Point {
        let mut origin = self.piece_position;
        while !self.board.collision_test(&self.piece, origin) {
            origin.y += 1;
        }
        origin.y -= 1;
        origin
    }

    fn render(&self, display: &mut Display) {
        // Render the board
        self.board.render(display);

        let width = self.piece.shape.len() as i32;

        // Render a ghost piece
        let ghost_origin = self.find_dropped_origin();

        // Render the currently falling piece
        for row in 0..width {
            for col in 0..width {
                if self.piece.shape[row as usize][col as usize] != 0 {
                    let x = (1 + 2 * (self.piece_position.x + col)) as u32;
                    let y = (self.piece_position.y + row) as u32;
                    let ghost_y = (ghost_origin.y + row) as u32;
                    display.set_pixel("*", x, ghost_y, self.piece.color, Color::Black);
                    display.set_pixel("*", x + 1, ghost_y, self.piece.color, Color::Black);
                    display.set_pixel(" ", x, y, self.piece.color, self.piece.color);
                    display.set_pixel(" ", x + 1, y, self.piece.color, self.piece.color);
                }
            }
        }
    }

    // Returns true if the piece could be moved
    fn move_piece(&mut self, x: i32, y: i32) -> bool {
        let new_position = Point{
            x: self.piece_position.x + x,
            y: self.piece_position.y + y,
        };
        if self.board.collision_test(&self.piece, new_position) {
            false
        } else {
            self.piece_position = new_position;
            true
        }
    }

    // Returns true if the piece was rotated
    fn rotate_piece(&mut self, direction: Direction) -> bool {
        let mut new_piece = self.piece.clone();
        new_piece.rotate(direction);

        if self.board.collision_test(&new_piece, self.piece_position) {
            false
        } else {
            self.piece = new_piece;
            true
        }
    }

    // Returns true if the new piece could be placed
    fn place_new_piece(&mut self) -> bool {
        let origin = Point{
            x: ((BOARD_WIDTH - (self.piece.shape.len() as u32)) / 2) as i32,
            y: 0,
        };
        if self.board.collision_test(&self.piece, origin) {
            false
        } else {
            self.piece_position = origin;
            true
        }
    }

    // Returns true if we were able to advance a piece (if false, game over)
    fn advance_piece(&mut self) -> bool {
        if !self.move_piece(0, 1) {
            self.board.lock_piece(&self.piece, self.piece_position);
            self.piece = self.piece_bag.pop();

            if !self.place_new_piece() {
                return false;
            }
        }

        true
    }

    fn drop_piece(&mut self) -> bool {
        while self.move_piece(0, 1) {}
        self.advance_piece()
    }

    fn keypress(&mut self, key: Key) {
        match key {
            Key::Left => self.move_piece(-1, 0),
            Key::Right => self.move_piece(1, 0),
            Key::Down => self.advance_piece(),
            Key::Up => self.rotate_piece(Direction::Left),
            Key::Space => self.drop_piece(),
            Key::Char('q') => self.rotate_piece(Direction::Left),
            Key::Char('e') => self.rotate_piece(Direction::Right),
            _ => false,
        };
    }
}

fn get_input(stdin: &mut std::io::Stdin) -> Option<Key> {
    use std::io::Read;

    let c = &mut [0u8];
    match stdin.read(c) {
        Ok(_) => {
            match std::str::from_utf8(c) {
                Ok("w") => Some(Key::Up),
                Ok("a") => Some(Key::Left),
                Ok("s") => Some(Key::Down),
                Ok("d") => Some(Key::Right),
                Ok(" ") => Some(Key::Space),
                Ok("\x03") => Some(Key::CtrlC),
                Ok("\x1b") => {
                    let code = &mut [0u8; 2];
                    match stdin.read(code) {
                        Ok(_) => {
                            match std::str::from_utf8(code) {
                                Ok("[A") => Some(Key::Up),
                                Ok("[B") => Some(Key::Down),
                                Ok("[C") => Some(Key::Right),
                                Ok("[D") => Some(Key::Left),
                                _ => None
                            }
                        },
                        Err(msg) => panic!(format!("could not read from standard in: {}", msg))
                    }
                },
                Ok(n) => Some(Key::Char(n.chars().next().unwrap())),
                _ => None
            }
        },
        Err(msg) => panic!(format!("could not read from standard in: {}", msg))
    }
}

fn main() {
    let display = &mut Display::new(BOARD_WIDTH * 2 + 2, BOARD_HEIGHT + 2);
    let game = &mut Game::new();
    let stdin = &mut std::io::stdin();

    let _restorer = terminal::set_terminal_raw_mode();

    loop {
        display.clear_buffer();
        game.render(display);
        display.render();

        match get_input(stdin) {
            Some(Key::Char('z')) | Some(Key::CtrlC) => break,
            Some(n) => game.keypress(n),
            _ => panic!("unrecognized key")
        }
    }
}
