# Rust Tetris

This is a simple console-based version of the game Tetris, written in Rust.

## How to play

    git clone https://github.com/mikejquinn/rust-tetris.git
    cd rust-tetris
    cargo run

Movement keys:

* Q - Rotate counter-clockwise
* E - Rotate clockwise
* A / S / D (or arrow keys) - move left, right, down
* Space - Drop piece to bottom of board

You've played Tetris before. Colored shapes (called "tetrominos") drop one at a time from the top of the game
board. The object of the game is to guide these pieces to the bottom of the board and position them such that
they completely fill horizontal rows. When a row is full, the blocks in that row are removed from the board,
and all set pieces above it will drop down to fill the space. The more lines you clear, the more points you
earn. However, as you earn points and advance to higher levels, the pieces will start dropping faster. The
game is over when the board fills up to the top of the screen and there is no room for place a new piece.

## Implementation

The game board is implemented as a multidimensional array of `Option<Color>`s, where `None` means that no
blocks occupy that cell. The currently active piece is stored and rendered separately from the board. Once the
piece has reached the bottom of the board and is "locked", its colors are copied over into the game board
matrix.

Game input and the piece drop timer are handled by two separate background threads which send game events to
the main thread over a [`channel`](https://doc.rust-lang.org/std/sync/mpsc/fn.channel.html). One thread simply
sleeps and sends `GameUpdate::Tick` events, while the other blocks on keyboard input and sends
`GameUpdate::KeyPress(Key)` events as they are detected. Rust's powerful `enum` type makes it very easy to
describe this communication from the background threads over a single channel without having to resort to a
more complex class hierarchy.

A simpler design for handling game input and the drop timer may have been to use Rust's
[`select!`](https://doc.rust-lang.org/std/macro.select!.html) on two separate channels (one sending input
events, one sending tick events). This would obviate the need for the `GameUpdate` type. Unfortunately, using
`select!` on channels is not supported in Rust's stable compiler.

## Next steps

There are still a few very small pieces I need to build before this is a feature-complete Tetris:
* Scoring
* Game levels with increasing difficulty
* [Wall-kick](http://tetris.wikia.com/wiki/Wall_kick)
* Game over
