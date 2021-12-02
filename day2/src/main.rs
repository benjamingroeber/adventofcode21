use helpers::{read_lines_parse, AocError, AocResult};
use std::str::FromStr;

type Unit = i32;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    Up(Unit),
    Down(Unit),
    Forward(Unit),
}

impl FromStr for Direction {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((direction, unit)) = s.split_once(' ') {
            let unit = unit.parse()?;
            match direction {
                "forward" => Ok(Direction::Forward(unit)),
                "up" => Ok(Direction::Up(unit)),
                "down" => Ok(Direction::Down(unit)),
                _ => Err(AocError::ParseStructError(format!(
                    "Unknown Direction {}",
                    direction
                ))),
            }
        } else {
            Err(AocError::ParseStructError(format!(
                "Invalid Direction {}",
                s
            )))
        }
    }
}

#[derive(Debug, Default)]
struct Submarine {
    depth: Unit,
    position: Unit,
}

impl Submarine {
    fn go(&mut self, direction: Direction) {
        match direction {
            Direction::Up(n) => self.depth -= n,
            Direction::Down(n) => self.depth += n,
            Direction::Forward(n) => self.position += n,
        }
    }

    fn go_n(&mut self, directions: &[Direction]) {
        for direction in directions {
            self.go(*direction)
        }
    }
}

#[derive(Debug, Default)]
struct Aimmarine {
    aim: Unit,
    depth: Unit,
    position: Unit,
}

impl Aimmarine {
    fn go(&mut self, direction: Direction) {
        match direction {
            // Again note that since you're on a submarine, down and up do the opposite of what
            // you might expect: "down" means aiming in the positive direction.
            Direction::Down(n) => self.aim += n,
            Direction::Up(n) => self.aim -= n,
            Direction::Forward(n) => {
                self.position += n;
                self.depth += self.aim * n;
            }
        }
    }

    fn go_n(&mut self, directions: &[Direction]) {
        for direction in directions {
            self.go(*direction)
        }
    }
}

fn main() -> AocResult<()> {
    let input: Vec<Direction> = read_lines_parse("day2/day2.input")?;

    let mut sub = Submarine::default();
    sub.go_n(&input);
    println!(
        "Position: {}, Depth: {}, Product = {}",
        sub.position,
        sub.depth,
        sub.position * sub.depth
    );

    let mut aim = Aimmarine::default();
    aim.go_n(&input);
    println!(
        "Position: {}, Depth: {}, Product = {}",
        aim.position,
        aim.depth,
        aim.position * aim.depth
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static DIRECTIONS: [Direction; 6] = [
        Direction::Forward(5),
        Direction::Down(5),
        Direction::Forward(8),
        Direction::Up(3),
        Direction::Down(8),
        Direction::Forward(2),
    ];
    #[test]
    fn test_parse() {
        let directions = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

        let parsed: Result<Vec<Direction>, _> = directions.lines().map(|l| l.parse()).collect();

        let correctly_parsed = parsed.unwrap();
        assert_eq!(correctly_parsed, DIRECTIONS)
    }

    #[test]
    fn example_part1() {
        let mut sub = Submarine::default();

        sub.go_n(&DIRECTIONS);

        assert_eq!(sub.position, 15);
        assert_eq!(sub.depth, 10);
    }

    #[test]
    fn example_part2() {
        let mut sub = Aimmarine::default();

        sub.go_n(&DIRECTIONS);

        assert_eq!(sub.position, 15);
        assert_eq!(sub.depth, 60);
    }
}
