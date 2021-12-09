use helpers::AocResult;
use std::fmt::{Display, Formatter};
use std::io::Read;
use std::time::Instant;

type Unit = u64;
const PARENT_REPRODUCTION_DAYS: usize = 7;

fn main() -> AocResult<()> {
    let start = Instant::now();
    let mut input = String::new();
    helpers::read_file_reader("day6/day6.input")?.read_to_string(&mut input)?;

    let numbers: Result<Vec<Unit>, _> = input
        .lines()
        .flat_map(|l| l.split(','))
        .map(|n| n.parse())
        .collect();

    let mut game = GameOfLanternfish::from_numbers(&numbers?);

    // Part 1:
    // How many lanternfish would there be after 80 days?
    let day1_days = 80;
    for _ in 0..day1_days {
        game.advance_one_day();
        // println!("{}", game);
    }
    println!("Count of Fish after {} days: {}", day1_days, game.count());

    // Part 2:
    // How many lanternfish would there be after 256 days?
    let day2_days = 256;
    for _ in 0..day2_days - day1_days {
        game.advance_one_day();
        // println!("{}", game);
    }
    println!("Count of Fish after {} days: {}", day2_days, game.count());
    let duration = start.elapsed();
    println!("Took: {:?}", duration);

    Ok(())
}

struct GameOfLanternfish {
    zero_day_bracket: usize,
    fishes: [Unit; PARENT_REPRODUCTION_DAYS],
    seven_day_fishes: Unit,
    eigth_day_fishes: Unit,
    newborn_fishes: Unit,
}

impl GameOfLanternfish {
    fn from_numbers(numbers: &[Unit]) -> Self {
        let mut game = GameOfLanternfish {
            zero_day_bracket: 0,
            fishes: Default::default(),
            seven_day_fishes: 0,
            eigth_day_fishes: 0,
            newborn_fishes: 0,
        };

        for n in numbers {
            game.fishes[*n as usize] += 1
        }

        game
    }

    fn new_parent_day_index(&self) -> usize {
        (self.zero_day_bracket + PARENT_REPRODUCTION_DAYS) % PARENT_REPRODUCTION_DAYS
    }

    fn advance_one_day(&mut self) {
        self.fishes[self.new_parent_day_index()] += self.seven_day_fishes;

        self.seven_day_fishes = self.eigth_day_fishes;
        self.eigth_day_fishes = self.newborn_fishes;

        self.zero_day_bracket = (self.zero_day_bracket + 1) % PARENT_REPRODUCTION_DAYS;
        self.newborn_fishes = self.fishes[self.zero_day_bracket]
    }

    fn count(&self) -> Unit {
        self.seven_day_fishes + self.eigth_day_fishes + self.fishes.iter().sum::<Unit>()
    }
}

impl Display for GameOfLanternfish {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "zero_day_idx: {}    ", self.zero_day_bracket)?;

        for i in 0..self.fishes.len() {
            let idx = (self.zero_day_bracket + i) % PARENT_REPRODUCTION_DAYS;
            write!(f, "{},", self.fishes[idx])?
        }

        write!(
            f,
            "{},{} => {}",
            self.seven_day_fishes, self.eigth_day_fishes, self.newborn_fishes
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        // This list means that the first fish has an internal timer of 3, the second fish has an
        // internal timer of 4, and so on until the fifth fish, which has an internal timer of 2.
        let mut game = GameOfLanternfish::from_numbers(&[3, 4, 3, 1, 2]);

        for _ in 0..18 {
            game.advance_one_day();
        }
        let day_18_count = game.count();

        for _ in 0..80 - 18 {
            game.advance_one_day();
        }
        let day_80_count = game.count();

        // After 18 days: 6,0,6,4,5,6,0,1,1,2,6,0,1,1,1,2,2,3,3,4,6,7,8,8,8,8
        // In this example, after 18 days, there are a total of 26 fish.
        assert_eq!(day_18_count, 26);
        // After 80 days, there would be a total of 5934.
        assert_eq!(day_80_count, 5934);
    }

    #[test]
    fn example_part2() {
        let mut game = GameOfLanternfish::from_numbers(&[3, 4, 3, 1, 2]);

        // After 256 days in the example above, there would be a total of 26984457539 lanternfish!
        for _ in 0..256 {
            game.advance_one_day();
        }

        assert_eq!(game.count(), 26984457539);
    }
}
