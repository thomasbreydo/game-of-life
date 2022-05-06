use std::{fmt, io};
use std::fmt::{Display, Formatter};
use std::thread::sleep;
use std::time::Duration;

pub struct Board {
    board: Vec<Vec<bool>>,
}

impl Board {
    pub fn new(nrows: usize, ncols: usize) -> Self {
        if nrows == 0 || ncols == 0 {
            panic!("non-zero number of rows and columns required");
        }
        Self { board: vec![vec![false; ncols]; nrows] }
    }

    pub fn from_file(filename: &str) -> io::Result<Self> {
        let s = std::fs::read_to_string(filename)?;
        let mut board: Vec<Vec<bool>> = Vec::new();
        for row in s.trim().split('\n') {
            let row: Vec<bool> = row.chars().map(|x| x == '#').collect();
            if !board.is_empty() && board[0].len() != row.len() {
                panic!("row lengths must match")
            }
            board.push(row);
        }
        Ok(Self { board })
    }

    pub fn turn_on(&mut self, row: usize, col: usize) {
        // todo introduce Result<()> for index out of bounds
        self.board[row][col] = true;
    }

    pub fn n_rows(&self) -> usize {
        self.board.len()
    }

    pub fn n_cols(&self) -> usize {
        self.board[0].len()
    }

    pub fn is_alive(&self, row: usize, col: usize) -> bool {
        self.board[row][col]
    }

    pub fn turn_off(&mut self, row: usize, col: usize) {
        self.board[row][col] = false;
    }

    fn n_neighbors(&self, row: usize, col: usize) -> usize {
        let mut count = 0;
        for drow in [self.n_rows() - 1, 0, 1] {
            for dcol in [self.n_cols() - 1, 0, 1] {
                if drow == 0 && dcol == 0 { continue; }
                let neigh_row = (row + drow) % self.n_rows();
                let neigh_col = (col + dcol) % self.n_cols();
                if self.is_alive(neigh_row, neigh_col) { count += 1; }
            }
        }
        count
    }

    pub fn step(&mut self) {
        let mut new_board = self.board.clone();
        for row in 0..self.n_rows() {
            for col in 0..self.n_cols() {
                let n = self.n_neighbors(row, col);
                if self.is_alive(row, col) && (n < 2 || n > 3) {
                    new_board[row][col] = false;
                } else if !self.is_alive(row, col) && (n == 3) {
                    new_board[row][col] = true;
                }
            }
        }
        self.board = new_board;
    }

    pub fn play(&mut self, delay: Duration) {
        loop {
            println!("\x1bc{}", self);
            self.step();
            sleep(delay);
        }
    }
}


impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut s = String::new();

        for row in 0..self.n_rows() {
            for col in 0..self.n_cols() {
                s.push(if self.is_alive(row, col) { '◼' } else { '◻' });
            }
            s.push('\n');
        };

        write!(f, "{}", s)
    }
}