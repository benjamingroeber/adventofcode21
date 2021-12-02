use helpers::{print_current_dir, AocResult};
use itertools::Itertools;
use std::ops::Add;

fn main() -> AocResult<()> {
    print_current_dir();
    let input: Vec<usize> = helpers::read_lines_parse("day1/day1.input")?;

    // count the number of times a depth measurement increases from the previous measurement
    println!(
        "Positive difference count: {}",
        count_positive_differences(input.iter())
    );

    // Considering every single measurement isn't as useful as you expected: there's just too
    // much noise in the data.
    //
    // Instead, consider sums of a three-measurement sliding window.
    // Your goal now is to count the number of times the sum of measurements in this sliding window
    // increases from the previous sum.
    let tripplet_sums = sum_windows_of_three(input.into_iter());
    println!(
        "Positive difference of windows of three count: {}",
        count_positive_differences(tripplet_sums)
    );

    Ok(())
}

fn count_positive_differences<T: PartialOrd + Clone>(i: impl Iterator<Item = T>) -> usize {
    i.tuple_windows::<(_, _)>()
        .filter(|(first, second)| second > first)
        .count()
}

fn sum_windows_of_three<T: Add<Output = T> + Clone>(
    i: impl Iterator<Item = T>,
) -> impl Iterator<Item = T> {
    i.tuple_windows::<(_, _, _)>().map(|w| w.0 + w.1 + w.2)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_NUMBERS: [i32; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    #[test]
    fn example_day1() {
        let differences = count_positive_differences(EXAMPLE_NUMBERS.iter());

        assert_eq!(differences, 7)
    }

    #[test]
    fn example_day2() {
        let sums_of_tripplets = sum_windows_of_three(EXAMPLE_NUMBERS.iter().cloned());

        let differences = count_positive_differences(sums_of_tripplets);

        assert_eq!(differences, 5)
    }
}
