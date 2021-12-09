use helpers::AocResult;
use itertools::Itertools;
use itertools::MinMaxResult::MinMax;
use std::io::Read;
use std::time::Instant;

type Unit = i32;

fn main() -> AocResult<()> {
    let start = Instant::now();
    let mut input = String::new();
    helpers::read_file_reader("day7/day7.input")?.read_to_string(&mut input)?;

    let numbers: Result<Vec<Unit>, _> = input
        .lines()
        .flat_map(|l| l.split(','))
        .map(|n| n.parse())
        .collect();

    let numbers = numbers?;
    let min_diff = minimize_difference(&numbers);
    println! {"Sum of minimum differences: {:?}", min_diff};

    let min_diff_exp = min_diff_exponential(&numbers);
    println! {"Sum of exponential minimum differences: {:?}", min_diff_exp};

    println!("Took: {:?}", start.elapsed());
    Ok(())
}

fn minimize_difference(nums: &[Unit]) -> Option<Unit> {
    if let MinMax(&min, &max) = nums.iter().minmax() {
        let mut min_sum: Unit = Unit::MAX;
        for i in min..max + 1 {
            let sum_diff: Unit = nums.iter().map(|n: &Unit| (*n - i).abs()).sum();
            min_sum = min_sum.min(sum_diff);
        }
        Some(min_sum)
    } else {
        None
    }
}

fn min_diff_exponential(nums: &[Unit]) -> Option<Unit> {
    if let MinMax(&min, &max) = nums.iter().minmax() {
        let mut min_sum: Unit = Unit::MAX;
        for i in min..max + 1 {
            let sum_diff: Unit = nums
                .iter()
                .map(|n: &Unit| {
                    let diff = (*n - i).abs();
                    (diff * (diff + 1)) / 2
                })
                .sum();
            min_sum = min_sum.min(sum_diff);
        }
        Some(min_sum)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let numbers = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

        let got_empty = minimize_difference(&[]);
        let got = minimize_difference(&numbers);

        assert_eq!(got_empty, None);
        assert_eq!(got, Some(37));
    }

    #[test]
    fn example_part2() {
        let numbers = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

        let got_empty = min_diff_exponential(&[]);
        let got = min_diff_exponential(&numbers);

        assert_eq!(got_empty, None);
        assert_eq!(got, Some(168));
    }
}
