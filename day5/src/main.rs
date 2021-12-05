use helpers::{read_lines_parse, AocError, AocResult};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;

type Unit = i32;

fn main() -> AocResult<()> {
    let input: Vec<Line> = read_lines_parse("day5/day5.input")?;

    let straight_grid = Grid::from_straight_lines_only(&input);

    // Part 1
    // Consider only horizontal and vertical lines
    // At how many points do at least two lines overlap?
    let points_with_overlapping = straight_grid.intersecting_point_count();
    println!(
        "Number of points with overlapping straight lines: {}",
        points_with_overlapping
    );

    // Part 2
    // You still need to determine the number of points where at least two lines overlap.
    // Consider all of the lines. At how many points do at least two lines overlap?
    let grid = Grid::from_lines(&input);
    let points_with_overlapping = grid.intersecting_point_count();
    println!(
        "Number of points with overlapping lines: {}",
        points_with_overlapping
    );

    Ok(())
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: Unit,
    y: Unit,
}

#[derive(Copy, Clone, Debug)]
struct Line {
    start: Point,
    end: Point,
}

// Because of the limits of the hydrothermal vent mapping system,
// the lines in your list will only ever be horizontal, vertical,
// or a diagonal line at exactly 45 degrees.
fn get_step_delta(first: Unit, second: Unit) -> Unit {
    match first.cmp(&second) {
        Ordering::Less => 1,
        Ordering::Equal => 0,
        Ordering::Greater => -1,
    }
}

impl Line {
    fn distinct_points(&self) -> impl Iterator<Item = Point> + '_ {
        PointIterator {
            slope: Point {
                x: get_step_delta(self.start.x, self.end.x),
                y: get_step_delta(self.start.y, self.end.y),
            },
            current: self.start,
            end: self.end,
            complete: false,
        }
    }
}

struct PointIterator {
    slope: Point,
    current: Point,
    end: Point,
    complete: bool,
}

impl Iterator for PointIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.complete {
            None
        } else {
            let next = Some(self.current);
            if self.current.x == self.end.x && self.current.y == self.end.y {
                self.complete = true;
            }
            self.current.x += self.slope.x;
            self.current.y += self.slope.y;
            next
        }
    }
}

static START_END_DELIM: &str = " -> ";

impl FromStr for Line {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((start, end)) = s.split_once(START_END_DELIM) {
            let start = Point::from_str(start)?;
            let end = Point::from_str(end)?;
            Ok(Line { start, end })
        } else {
            Err(AocError::ParseStructError(format!(
                "Missing delimiter '{}' in Line '{}'",
                START_END_DELIM, s
            )))
        }
    }
}

const POINT_DELIM: char = ',';
impl FromStr for Point {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((x, y)) = s.split_once(POINT_DELIM) {
            let x = x.parse()?;
            let y = y.parse()?;
            Ok(Point { x, y })
        } else {
            Err(AocError::ParseStructError(format!(
                "Missing delimiter '{}' in Point '{}'",
                POINT_DELIM, s
            )))
        }
    }
}

// Grid representing the number of lines present at each Point
struct Grid {
    data: HashMap<Point, usize>,
}

impl Grid {
    fn from_straight_lines_only(lines: &[Line]) -> Self {
        Self::construct_grid(straight_lines(lines))
    }
    fn from_lines(lines: &[Line]) -> Self {
        Self::construct_grid(lines.iter())
    }
    fn construct_grid<'a>(lines: impl Iterator<Item = &'a Line>) -> Self {
        let mut grid = HashMap::new();
        for point in lines.flat_map(|l| l.distinct_points()) {
            let counter = grid.entry(point).or_insert(0);
            *counter += 1;
        }
        Grid { data: grid }
    }
    fn intersecting_point_count(&self) -> usize {
        self.data.values().filter(|&&n| n > 1).count()
    }
}

fn straight_lines(lines: &[Line]) -> impl Iterator<Item = &Line> {
    lines
        .iter()
        .filter(|line| line.start.x == line.end.x || line.start.y == line.end.y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1_line_points() {
        // An entry like 1,1 -> 1,3 covers points 1,1, 1,2, and 1,3.
        // An entry like 9,7 -> 7,7 covers points 9,7, 8,7, and 7,7.
        let line1 = Line {
            start: Point { x: 1, y: 1 },
            end: Point { x: 1, y: 3 },
        };

        let line2 = Line {
            start: Point { x: 9, y: 7 },
            end: Point { x: 7, y: 7 },
        };

        let l1_c: Vec<_> = line1.distinct_points().collect();
        let l2_c: Vec<_> = line2.distinct_points().collect();

        assert_eq!(
            l1_c,
            &[
                Point { x: 1, y: 1 },
                Point { x: 1, y: 2 },
                Point { x: 1, y: 3 }
            ]
        );

        assert_eq!(
            l2_c,
            &[
                Point { x: 9, y: 7 },
                Point { x: 8, y: 7 },
                Point { x: 7, y: 7 }
            ]
        )
    }

    #[test]
    fn example_part1_overlapping() {
        let input: Vec<Line> = read_lines_parse("day5.testinput").unwrap();

        let grid = Grid::from_straight_lines_only(&input);
        let points_with_overlapping = grid.intersecting_point_count();

        assert_eq!(points_with_overlapping, 5)
    }

    #[test]
    fn example_part2_diagonal_line_points() {
        //An entry like 1,1 -> 3,3 covers points 1,1, 2,2, and 3,3.
        //An entry like 9,7 -> 7,9 covers points 9,7, 8,8, and 7,9.

        let l1 = Line {
            start: Point { x: 1, y: 1 },
            end: Point { x: 3, y: 3 },
        };
        let d_points1: Vec<_> = l1.distinct_points().collect();
        let l2 = Line {
            start: Point { x: 9, y: 7 },
            end: Point { x: 7, y: 9 },
        };
        let d_points2: Vec<_> = l2.distinct_points().collect();

        assert_eq!(
            d_points1,
            &[
                Point { x: 1, y: 1 },
                Point { x: 2, y: 2 },
                Point { x: 3, y: 3 }
            ]
        );
        assert_eq!(
            d_points2,
            &[
                Point { x: 9, y: 7 },
                Point { x: 8, y: 8 },
                Point { x: 7, y: 9 }
            ]
        );
    }

    #[test]
    fn example_part2_overlapping() {
        let input: Vec<Line> = read_lines_parse("day5.testinput").unwrap();

        let grid = Grid::from_lines(&input);
        let points_with_overlapping = grid.intersecting_point_count();

        assert_eq!(points_with_overlapping, 12)
    }
}
