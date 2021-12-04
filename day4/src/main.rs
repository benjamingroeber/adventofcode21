use helpers::{read_file, AocError, AocResult, Grid};
use std::fmt::{Display, Formatter};
use std::io::Read;
use std::mem::swap;
use std::str::FromStr;

// Bingo is played on a set of boards each consisting of a 5x5 grid of numbers.
// Numbers are chosen at random, and the chosen number is marked on all boards on which it appears.
// (Numbers may not appear on all boards.) If all numbers in any row or any column of a board are
// marked, that board wins. (Diagonals don't count.)
const BINGO_BOARD_GRID: usize = 5;
type Unit = u32;
fn main() -> AocResult<()> {
    let mut data = String::new();
    // read_file("day4/day4.testinput")?.read_to_string(&mut data)?;
    read_file("day4/day4.input")?.read_to_string(&mut data)?;

    let (numbers, mut game) = parse_input(&data)?;

    // Part 1
    // The score of the winning board can now be calculated. Start by finding the sum of all
    // unmarked numbers on that board. Then, multiply that sum by the number that was just called
    // when the board won, to get the final score.
    let first_winner = game
        .play(&numbers)
        .ok_or_else(|| AocError::GridError("With these numbers, nobody wins!".to_string()))?;
    println!("Part 1 First Game\n{}", first_winner);

    // Part 2
    // You aren't sure how many bingo boards a giant squid could play at once, so rather than waste
    // time counting its arms, the safe thing to do is to figure out which board will win last and
    // choose that one. That way, no matter which boards it picks, it will win for sure.
    let last_winner = game
        .play_to_end(&numbers[first_winner.turns..])
        .ok_or_else(|| {
            AocError::GridError("With these Numbers, only one Board winds!".to_string())
        })?;
    println!("Part 2 Last Game\n{}", last_winner);

    Ok(())
}

fn parse_input(s: &str) -> AocResult<(Vec<Unit>, BingoGame)> {
    if let Some((numbers, boards)) = s.split_once("\n\n") {
        let numbers: Result<Vec<Unit>, _> = numbers.split(',').map(|n| n.parse()).collect();
        let boards: Result<Vec<_>, _> = boards
            .split("\n\n")
            .map(|s| BingoBoard::from_str(s).map(Some))
            .collect();
        Ok((numbers?, BingoGame { boards: boards? }))
    } else {
        Err(AocError::ParseStructError(
            "Could not split numbers from boards".to_string(),
        ))
    }
}

pub struct BingoGame {
    boards: Vec<Option<BingoBoard>>,
}

pub struct Winner {
    turns: usize,
    winning_number: Unit,
    winning_board: BingoBoard,
}

impl Display for Winner {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let sum_unmarked = self.winning_board.sum_unmarked();
        writeln!(
            f,
            "Game took {} turns\nSum of unmarked fields: {}\nWinning number {}\nProduct:{}",
            self.turns,
            sum_unmarked,
            self.winning_number,
            self.winning_number * sum_unmarked
        )
    }
}

impl BingoGame {
    // play bingo until all boards won
    // returns the last winning board
    pub fn play_to_end(&mut self, mut numbers: &[Unit]) -> Option<Winner> {
        let mut last_winner = None;
        while let Some(winner) = self.play(numbers) {
            numbers = &numbers[winner.turns..];
            last_winner = Some(winner);
        }
        last_winner
    }
    // returns Some(idx) of the winning board
    // None if nobody wins
    pub fn play(&mut self, numbers: &[Unit]) -> Option<Winner> {
        for (i, &n) in numbers.iter().enumerate() {
            if let Some(idx) = self.play_number(n) {
                let mut winner = None;
                swap(&mut self.boards[idx], &mut winner);

                return Some(Winner {
                    turns: i,
                    winning_number: n,
                    winning_board: winner.unwrap(),
                });
            }
        }
        None
    }

    fn play_number(&mut self, number: Unit) -> Option<usize> {
        for (i, board) in self.boards.iter_mut().enumerate() {
            if let Some(board) = board {
                board.cross(number);
                if board.is_bingo() {
                    return Some(i);
                }
            }
        }
        None
    }
}

#[derive(Copy, Clone, Debug)]
enum BingoField {
    Open(Unit),
    Crossed(Unit),
}

pub struct BingoBoard {
    data: Grid<BingoField>,
}

impl BingoBoard {
    fn cross(&mut self, number: Unit) {
        for uncrossed_number in self
            .data
            .iter_mut()
            .filter(|f| matches! {f, BingoField::Open(u) if *u == number })
        {
            *uncrossed_number = BingoField::Crossed(number)
        }
    }

    fn is_bingo(&self) -> bool {
        (0..BINGO_BOARD_GRID).any(|i| {
            self.data
                .iter_col(i)
                .all(|f| matches!(f, BingoField::Crossed(_)))
                || self
                    .data
                    .iter_row(i)
                    .all(|f| matches!(f, BingoField::Crossed(_)))
        })
    }

    fn sum_unmarked(&self) -> Unit {
        self.data
            .iter()
            .filter_map(|f| match f {
                BingoField::Open(n) => Some(n),
                BingoField::Crossed(_) => None,
            })
            .sum()
    }
}

impl FromStr for BingoBoard {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers: Result<Vec<_>, _> = s
            .split('\n')
            .flat_map(|line| line.split_ascii_whitespace())
            .map(|n| n.parse::<Unit>().map(BingoField::Open))
            .collect();

        let board = BingoBoard {
            data: Grid::from_slice(&numbers.unwrap(), BINGO_BOARD_GRID)?,
        };
        Ok(board)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let mut data = String::new();
        read_file("day4.testinput")
            .unwrap()
            .read_to_string(&mut data)
            .unwrap();

        let (numbers, mut game) = parse_input(&data).unwrap();
        let winner = game.play(&numbers).unwrap();

        // The score of the winning board can now be calculated. Start by finding the sum of all
        // unmarked numbers on that board; in this case, the sum is 188. Then, multiply that sum by
        // the number that was just called when the board won, 24, to get the final score,
        // 188 * 24 = 4512.
        assert_eq!(winner.winning_board.sum_unmarked(), 188);
        assert_eq!(winner.winning_number, 24);
    }

    #[test]
    fn example_part2() {
        let mut data = String::new();
        read_file("day4.testinput")
            .unwrap()
            .read_to_string(&mut data)
            .unwrap();

        let (numbers, mut game) = parse_input(&data).unwrap();

        let last_winner = game.play_to_end(&numbers).unwrap();

        // In the above example, the second board is the last to win, which happens after 13 is
        // eventually called and its middle column is completely marked. If you were to keep playing
        // until this point, the second board would have a sum of unmarked numbers equal to 148
        // for a final score of 148 * 13 = 1924.
        assert_eq!(last_winner.winning_board.sum_unmarked(), 148);
        assert_eq!(last_winner.winning_number, 13);
    }
}
