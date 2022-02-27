#![allow(dead_code)]
#![deny(clippy::all, clippy::pedantic)]
use std::{
    fs::File,
    io::{self, Read},
};

fn main() -> io::Result<()> {
    let mut f = File::open("src/day4/input.txt")?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    let game = Bingo::new(&buf);
    let score = game.play();
    println!("part1: {score}");

    Ok(())
}

struct Bingo {
    drawn: Vec<Number>,
    boards: Vec<Board>,
}

impl Bingo {
    fn new(input: &str) -> Self {
        let mut parts = input.split("\n\n");
        let drawn = parts.next().unwrap();
        let drawn = drawn.split(',').map(|n| n.parse().unwrap()).collect();

        let boards = parts.map(|p| Board::new(p)).collect();

        Self { drawn, boards }
    }

    fn play(&self) -> Number {
        for i in 1..self.drawn.len() {
            match self
                .boards
                .iter()
                .filter_map(|b| b.score(&self.drawn[0..i]))
                .next()
            {
                Some(score) => {
                    return score;
                }
                None => {}
            }
        }
        0
    }
}

/// A bingo [Board] number
type Number = usize;

/// A bingo [Board] with a 5x5 grid of [Number]s
#[derive(Debug)]
struct Board {
    numbers: Vec<Number>,
}

impl Board {
    const ROWS: usize = 5;
    const COLS: usize = 5;

    /// Create a new bingo [Board] from an `input` str slice.
    ///
    /// Creation can fail if there are non-numeric values in the input or if there are not 25 [Number]s provided.
    pub fn new(input: &str) -> Self {
        let mut numbers = Vec::with_capacity(Self::ROWS * Self::COLS);

        for row in input.split('\n') {
            for element in row.split_whitespace() {
                let number = element.parse().expect("parse number");
                numbers.push(number);
            }
        }

        Self { numbers }
    }

    /// Get the [Number] on the bingo [Board] at a particular `row` and `col`.
    ///
    /// Returns `None` if the `row` and `col` combination is invalid.
    fn get_number(&self, row: usize, col: usize) -> Option<Number> {
        let index = row * Self::COLS + col;
        if row >= Self::ROWS || col >= Self::COLS {
            None
        } else {
            self.numbers.get(index).copied()
        }
    }

    pub fn score(&self, drawn: &[Number]) -> Option<Number> {
        let marked = self.find_marked(drawn);
        if self.has_complete_row(&marked) || self.has_complete_col(&marked) {
            let sum: Number = self
                .numbers
                .iter()
                .enumerate()
                .filter_map(|(i, n)| if marked[i] { None } else { Some(*n) })
                .sum();
            Some(sum * drawn[drawn.len() - 1])
        } else {
            None
        }
    }

    /// Find all of the marked bingo [Number]s given the `drawn` numbers and return a boolean map corresponding to the marked [Number]s.
    fn find_marked(&self, drawn: &[Number]) -> [bool; Self::ROWS * Self::COLS] {
        let mut marked = [false; Self::ROWS * Self::COLS];

        self.numbers
            .iter()
            .enumerate()
            .for_each(|(i, n)| marked[i] = drawn.contains(n));

        marked
    }

    fn has_complete_row(&self, marked: &[bool]) -> bool {
        for row in 0..Self::ROWS {
            let i = row * Self::COLS;
            let j = i + Self::COLS;
            let found = (i..j).into_iter().all(|i| marked[i]);
            if found {
                return true;
            }
        }

        false
    }

    fn has_complete_col(&self, marked: &[bool]) -> bool {
        for col in 0..Self::COLS {
            let found = (col..Self::ROWS * Self::COLS)
                .step_by(Self::ROWS)
                .into_iter()
                .all(|i| marked[i]);
            if found {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::{Bingo, Board};

    const INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7";

    const BOARD_INPUT: &str = "22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19";

    #[test]
    fn test_board_new() {
        let board = Board::new(BOARD_INPUT);

        assert_eq!(board.numbers[0], 22);
        assert_eq!(board.numbers[24], 19);
    }

    #[test]
    fn test_board_get_number() {
        let board = Board::new(BOARD_INPUT);

        assert_eq!(board.get_number(0, 0), Some(22));
        assert_eq!(board.get_number(4, 4), Some(19));
        assert_eq!(board.get_number(7, 0), None);
        assert_eq!(board.get_number(0, 7), None);
    }

    #[test]
    fn test_bingo() {
        let bingo = Bingo::new(INPUT);
        assert_eq!(bingo.play(), 4512);
    }
}
