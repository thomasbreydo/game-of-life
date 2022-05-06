use std::time::Duration;

use crate::board::Board;

mod board;

static FILENAME: &str = "/Users/thomasbreydo/dev/game-of-life/src/board.txt";

fn main() {
    let mut board = Board::from_file(FILENAME).expect("no file found");
    board.play(Duration::from_millis(50));
}
