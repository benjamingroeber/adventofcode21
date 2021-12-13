use helpers::{read_file_string, AocError, AocResult, Grid};
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

type Unit = usize;

#[derive(Debug)]
struct Point(Unit, Unit);

//<x>,<y>
const POINT_DELIM: char = ',';
impl FromStr for Point {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((x, y)) = s.split_once(POINT_DELIM) {
            let x = x.parse()?;
            let y = y.parse()?;
            Ok(Point(x, y))
        } else {
            Err(AocError::ParseStructError(format!(
                "Delimiter '{}' missing in Line '{}'",
                POINT_DELIM, s
            )))
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Fold {
    Y(usize),
    X(usize),
}

static FOLD_PREFIX: &str = "fold along ";
const FOLD_DELIM: char = '=';
//fold along y=<y>
impl FromStr for Fold {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fold = s.strip_prefix(FOLD_PREFIX).ok_or_else(|| {
            AocError::ParseStructError(format!(
                "Fold instruction '{}', missing prefix '{}'",
                s, FOLD_PREFIX
            ))
        })?;
        if let Some((direction, distance)) = fold.split_once(FOLD_DELIM) {
            let distance = distance.parse()?;
            match direction {
                "x" | "X" => Ok(Fold::X(distance)),
                "y" | "Y" => Ok(Fold::Y(distance)),
                _ => Err(AocError::ParseStructError(format!(
                    "Unknown fold direction '{}' in '{}'",
                    direction, s
                ))),
            }
        } else {
            Err(AocError::ParseStructError(format!(
                "Fold '{}' missing delimiter '{}'",
                s, FOLD_DELIM
            )))
        }
    }
}

#[derive(Copy, Clone)]
enum Dot {
    Marked,
    Empty,
}

impl Debug for Dot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            Dot::Marked => 'x',
            Dot::Empty => '.',
        };
        write!(f, "{}", symbol)
    }
}

struct Paper {
    grid: Grid<Dot>,
}

impl Debug for Paper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.grid)
    }
}

impl Paper {
    fn with_points(points: &[Point]) -> Option<Self> {
        if let Some(max_x) = points.iter().map(|p| p.0).max() {
            if let Some(max_y) = points.iter().map(|p| p.1).max() {
                let grid = Grid::with_default(max_x + 1, max_y + 1, Dot::Empty);
                let mut paper = Paper { grid };
                for p in points {
                    // if starting point fails to mark, there is something fishy going on
                    if !paper.mark(p.0, p.1) {
                        return None;
                    }
                }
                return Some(paper);
            }
        }
        // Empty
        None
    }

    // returns true if a spot was marked
    //         false if index out of bounds
    fn mark(&mut self, x: usize, y: usize) -> bool {
        if let Some(dot) = self.grid.get_mut(x, y) {
            *dot.value = Dot::Marked;
            true
        } else {
            false
        }
    }

    fn unmark(&mut self, x: usize, y: usize) -> bool {
        if let Some(dot) = self.grid.get_mut(x, y) {
            *dot.value = Dot::Empty;
            true
        } else {
            false
        }
    }

    pub fn fold(&mut self, fold: Fold) -> AocResult<()> {
        match fold {
            Fold::Y(pivot) => self.fold_y(pivot),
            Fold::X(pivot) => self.fold_x(pivot),
        }
    }

    fn fold_y(&mut self, pivot_y: usize) -> AocResult<()> {
        for (offset, y) in (pivot_y..self.grid.row_count()).enumerate() {
            for x in 0..self.grid.column_count() {
                if let Some(Dot::Marked) = self.grid.get(x, y).map(|g| g.value) {
                    if !self.mark(x, pivot_y - offset) {
                        return Err(AocError::ChallengeError(format!(
                            "Folding along y={} out of bounds on {},{}",
                            pivot_y, x, y
                        )));
                    }
                    self.unmark(x, y);
                }
            }
        }
        Ok(())
    }

    fn fold_x(&mut self, pivot_x: usize) -> AocResult<()> {
        for y in 0..self.grid.row_count() {
            for (offset, x) in (pivot_x..self.grid.column_count()).enumerate() {
                if let Some(Dot::Marked) = self.grid.get(x, y).map(|g| g.value) {
                    if !self.mark(pivot_x - offset, y) {
                        return Err(AocError::ChallengeError(format!(
                            "Folding along y={} out of bounds on {},{}",
                            pivot_x, x, y
                        )));
                    }
                    self.unmark(x, y);
                }
            }
        }
        Ok(())
    }

    fn count_dots(&self) -> usize {
        self.grid
            .iter()
            .filter(|d| match d {
                Dot::Marked => true,
                Dot::Empty => false,
            })
            .count()
    }
}

fn main() -> AocResult<()> {
    let input = read_file_string("day13/day13.input")?;
    if let Some((points, folds)) = input.split_once("\n\n") {
        let points: Vec<Point> = points
            .lines()
            .map(Point::from_str)
            .collect::<AocResult<_>>()?;
        let folds: Vec<Fold> = folds
            .lines()
            .map(Fold::from_str)
            .collect::<AocResult<_>>()?;

        let mut paper = Paper::with_points(&points).ok_or_else(|| {
            AocError::ChallengeError("Something went wrong during paper creation".to_string())
        })?;

        let mut fold_iter = folds.iter();

        if let Some(first_fold) = fold_iter.next() {
            paper.fold(*first_fold)?;
            println!("Total dot count after first fold: {}", paper.count_dots());
        }

        for f in fold_iter {
            paper.fold(*f)?;
        }

        for y in 0..10 {
            for dot in paper.grid.iter_row(y).take(50) {
                let print_clearly = match dot.value {
                    Dot::Marked => '#',
                    Dot::Empty => ' ',
                };
                print!("{}", print_clearly);
            }
            println!();
        }
    }

    // println!("{:?}", input);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "day13.testinput";

    #[test]
    fn example_part1() {
        let input = helpers::read_file_string(TEST_INPUT).unwrap();
        let (points, folds) = input.split_once("\n\n").unwrap();
        let points: Vec<Point> = points
            .lines()
            .map(Point::from_str)
            .collect::<AocResult<_>>()
            .unwrap();
        let folds: Vec<Fold> = folds
            .lines()
            .map(Fold::from_str)
            .collect::<AocResult<_>>()
            .unwrap();

        let mut paper = Paper::with_points(&points).unwrap();

        paper.fold(folds[0]).unwrap();
        let fold1_count = paper.count_dots();

        paper.fold(folds[1]).unwrap();
        let fold2_count = paper.count_dots();

        assert_eq!(fold1_count, 17);
        assert_eq!(fold2_count, 16);
    }
}
