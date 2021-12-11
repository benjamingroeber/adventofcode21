use helpers::{read_file_string, AocError, AocResult, Grid};
use std::str::FromStr;

type Unit = u32;
const FLASH_THRESHOLD: u32 = 9;

#[derive(Debug)]
struct Octopusses {
    state: Grid<Unit>,
}

impl Octopusses {
    // returns the number of flashes that ocurred during this step
    fn flash(&mut self, x: usize, y: usize) {
        let middle = self
            .state
            .get_mut(x, y)
            .expect("Flashed Items must be inside dimensions");
        // any octopus that flashed during this step has its energy level set to 0,
        // as it used all of its energy to flash.
        *middle.value = 0;
        for (x, y) in self.state.surrounding_indexes(x, y) {
            let surrounding_octopus = self.state.get_mut(x, y).unwrap();
            // The only octopusses that are 0 have already exploded a round and must be ignored
            if *surrounding_octopus.value > 0 {
                *surrounding_octopus.value += 1
            }
        }
    }
    fn step(&mut self) -> usize {
        // First, the energy level of each octopus increases by 1.
        // After this step it is impossible to have octopusses with 0 energy.
        for octopus in self.state.iter_mut() {
            *octopus += 1
        }

        // Then, any octopus with an energy level greater than 9 flashes.
        // This increases the energy level of all adjacent octopuses by 1, including octopuses that
        // are diagonally adjacent. If this causes an octopus to have an energy level greater than 9,
        // it also flashes. This process continues as long as new octopuses keep having their energy
        // level increased beyond 9. (An octopus can only flash at most once per step.)
        let mut flashes = 0;
        let (x_max, y_max) = self.state.dimensions();
        while self.state.iter().any(|p| *p > FLASH_THRESHOLD) {
            for y in 0..y_max {
                for x in 0..x_max {
                    let value = self
                        .state
                        .get(x, y)
                        .expect("Item must be inside valid dimensions");
                    if *value.value > FLASH_THRESHOLD {
                        flashes += 1;
                        self.flash(x, y)
                    }
                }
            }
        }
        flashes
    }

    // returns the number of steps required to reach the synchronized flashing
    fn step_until_synchronized_flash(&mut self) -> usize {
        let mut steps = 0;
        let dimensions = self.state.dimensions();
        let octocount = dimensions.0 * dimensions.1;
        let mut flashes = 0;
        while flashes != octocount {
            steps += 1;
            flashes = self.step();
        }
        steps
    }
}

impl FromStr for Octopusses {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x_length = s
            .lines()
            .next()
            .map(|l| l.chars().count())
            .ok_or_else(|| AocError::ParseStructError("No Octopusses found!".to_string()))?;

        let numbers: Vec<Unit> = s
            .lines()
            .flat_map(|l| l.chars())
            .filter_map(|c| c.to_digit(10))
            .collect();

        let state = Grid::from_slice(&numbers, x_length)?;
        Ok(Octopusses { state })
    }
}

fn main() -> AocResult<()> {
    let input = read_file_string("day11/day11.input")?;
    // Part 1
    // Given the starting energy levels of the dumbo octopuses in your cavern, simulate 100 steps.
    // How many total flashes are there after 100 steps?
    let mut octopy1 = Octopusses::from_str(&input)?;
    let flash_sum: usize = (0..100).map(|_| octopy1.step()).sum();
    println!("Number of flashes after 100 steps: {}", flash_sum);

    // Part 2
    // If you can calculate the exact moments when the octopuses will all flash simultaneously, you
    // should be able to navigate through the cavern. What is the first step during which all
    // octopuses flash?
    let mut octopy2 = Octopusses::from_str(&input)?;
    let steps = octopy2.step_until_synchronized_flash();
    println!("The first synchronized flash happens at step: {}", steps);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let input = read_file_string("day11.testinput").unwrap();
        let mut octopy = Octopusses::from_str(&input).unwrap();

        // After 100 steps, there have been a total of 1656 flashes.
        let flash_sum: usize = (0..100).map(|_| octopy.step()).sum();

        assert_eq!(flash_sum, 1656)
    }

    #[test]
    fn example_part2() {
        assert!(true);

        let input = read_file_string("day11.testinput").unwrap();
        let mut octopy = Octopusses::from_str(&input).unwrap();

        // In the example above, the first time all octopuses flash simultaneously is step 195:
        let step_count = octopy.step_until_synchronized_flash();

        assert_eq!(step_count, 195)
    }
}
