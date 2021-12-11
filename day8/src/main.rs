use helpers::{AocError, AocResult};
use std::collections::HashSet;
use std::str::FromStr;

const ONE_SEGMENTS: usize = 2;
const FOUR_SEGMENTS: usize = 4;
const SEVEN_SEGMENTS: usize = 3;
const EIGHT_SEGMENTS: usize = 7;
const TWO_THREE_FIVE_SEGMENTS: usize = 5;
const ZERO_SIX_NINE_SEGMENTS: usize = 6;

#[derive(Clone, Debug)]
struct DigitDisplay {
    signal_patterns: Vec<Pattern>,
    output: Vec<Pattern>,
}

impl FromStr for DigitDisplay {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((signal, output)) = s.split_once(" | ") {
            let display = DigitDisplay {
                signal_patterns: signal
                    .split_ascii_whitespace()
                    .map(|l| l.chars().collect())
                    .collect(),
                output: output
                    .split_ascii_whitespace()
                    .map(|l| l.chars().collect())
                    .collect(),
            };
            Ok(display)
        } else {
            Err(AocError::ParseStructError(format!(
                "Separator missing in Digit Display '{}'",
                s
            )))
        }
    }
}

fn main() -> AocResult<()> {
    let displays: Vec<DigitDisplay> = helpers::read_lines_parse("day8/day8.input")?;
    // Part 1
    // Because the digits 1, 4, 7, and 8 each use a unique number of segments, you should be able to
    // tell which combinations of signals correspond to those digits.
    // In the output values, how many times do digits 1, 4, 7, or 8 appear?
    let count_digits_with_unique_number: usize = displays
        .iter()
        .map(|d| count_unique_patterns(&d.output))
        .sum();
    println!(
        "Digits with unique numbers: {}",
        count_digits_with_unique_number
    );

    // Part 2
    // For each entry, determine all of the wire/segment connections and decode the four-digit
    // output values. What do you get if you add up all of the output values?
    let mut sum = 0;
    for display in displays {
        sum += display.decode().ok_or_else(|| {
            AocError::ParseStructError(format!("Display could not be solved: {:?}", display))
        })?;
    }
    println!("Sum of output values: {}", sum);
    Ok(())
}

fn count_unique_patterns(patterns: &[Pattern]) -> usize {
    patterns
        .iter()
        .filter(|p| {
            let segment_count = p.len();
            segment_count == ONE_SEGMENTS
                || segment_count == FOUR_SEGMENTS
                || segment_count == SEVEN_SEGMENTS
                || segment_count == EIGHT_SEGMENTS
        })
        .count()
}

type Pattern = HashSet<char>;
type Solution = [Pattern; 10];

impl DigitDisplay {
    //   0:      1:      2:      3:      4:
    //  aaaa    ....    aaaa    aaaa    ....
    // b    c  .    c  .    c  .    c  b    c
    // b    c  .    c  .    c  .    c  b    c
    //  ....    ....    dddd    dddd    dddd
    // e    f  .    f  e    .  .    f  .    f
    // e    f  .    f  e    .  .    f  .    f
    //  gggg    ....    gggg    gggg    ....
    //
    //   5:      6:      7:      8:      9:
    //  aaaa    aaaa    aaaa    aaaa    aaaa
    // b    .  b    .  .    c  b    c  b    c
    // b    .  b    .  .    c  b    c  b    c
    //  dddd    dddd    ....    dddd    dddd
    // .    f  e    f  .    f  e    f  .    f
    // .    f  e    f  .    f  e    f  .    f
    //  gggg    gggg    ....    gggg    gggg

    fn find_pattern_with_length(&self, len: usize) -> Option<Pattern> {
        self.signal_patterns
            .iter()
            .find(|p| p.len() == len)
            .cloned()
    }

    fn find_pattern_with_condition<P>(&self, len: usize, mut condition: P) -> Option<Pattern>
    where
        P: FnMut(&Pattern) -> bool,
    {
        self.signal_patterns
            .iter()
            .find(|p| p.len() == len && condition(p))
            .cloned()
    }

    fn solve(&self) -> Option<Solution> {
        // Unique 1, 4, 7, 8
        let one_pattern = self.find_pattern_with_length(ONE_SEGMENTS)?;
        let four_pattern = self.find_pattern_with_length(FOUR_SEGMENTS)?;
        let seven_pattern = self.find_pattern_with_length(SEVEN_SEGMENTS)?;
        let eight_pattern = self.find_pattern_with_length(EIGHT_SEGMENTS)?;

        // 6 = six chars and, one in common with 1
        let six_pattern = self.find_pattern_with_condition(ZERO_SIX_NINE_SEGMENTS, |p| {
            one_pattern.iter().filter(|op| p.contains(op)).count() == 1
        })?;

        // 5 = five chars and all in common with 6
        let five_pattern = self.find_pattern_with_condition(TWO_THREE_FIVE_SEGMENTS, |p| {
            p.iter().all(|s| six_pattern.contains(s))
        })?;

        // e = other char in 6 ; used for two and nine
        let e_signal = six_pattern.iter().find(|s| !five_pattern.contains(s))?;

        // 2 = five chars and contains e
        let two_pattern =
            self.find_pattern_with_condition(TWO_THREE_FIVE_SEGMENTS, |p| p.contains(e_signal))?;

        // 9 = six chars and not contains e
        let nine_pattern =
            self.find_pattern_with_condition(ZERO_SIX_NINE_SEGMENTS, |p| !p.contains(e_signal))?;

        // 3 = remaining five chars
        let three_pattern = self.find_pattern_with_condition(TWO_THREE_FIVE_SEGMENTS, |p| {
            *p != two_pattern && *p != five_pattern
        })?;

        // 0 = remaining six chars
        let zero_pattern = self.find_pattern_with_condition(ZERO_SIX_NINE_SEGMENTS, |p| {
            *p != six_pattern && *p != nine_pattern
        })?;

        Some([
            zero_pattern,
            one_pattern,
            two_pattern,
            three_pattern,
            four_pattern,
            five_pattern,
            six_pattern,
            seven_pattern,
            eight_pattern,
            nine_pattern,
        ])
    }

    pub fn decode(&self) -> Option<usize> {
        self.solve().map(|solution| {
            let mut sum = 0;
            for (i, n) in self.output.iter().rev().enumerate() {
                let digit = solution
                    .iter()
                    .enumerate()
                    .find(|(_, p)| *p == n)
                    .map(|(n, _)| n)
                    .unwrap();
                sum += digit * 10_usize.pow(i as u32);
            }
            sum
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        // Patterns correspond 1:1 to the actual values
        let zero: Pattern = "abcefg".chars().collect();
        let one: Pattern = "cf".chars().collect();
        let two: Pattern = "acdeg".chars().collect();
        let three: Pattern = "acdfg".chars().collect();
        let four: Pattern = "bcdf".chars().collect();
        let five: Pattern = "abdfg".chars().collect();
        let six: Pattern = "abdefg".chars().collect();
        let seven: Pattern = "acf".chars().collect();
        let eight: Pattern = "abcdefg".chars().collect();
        let nine: Pattern = "abcdfg".chars().collect();

        let patterns = [zero, one, two, three, four, five, six, seven, eight, nine];
        let display = DigitDisplay {
            signal_patterns: Vec::from(patterns.clone()),
            output: vec![],
        };
        let solved = display.solve().unwrap();
        for (i, p) in patterns.iter().enumerate() {
            assert_eq!(solved[i], *p)
        }
    }

    #[test]
    fn test_part1() {
        let displays: Vec<DigitDisplay> = helpers::read_lines_parse("day8.testinput").unwrap();

        let count_digits_with_unique_number: usize = displays
            .iter()
            .map(|d| count_unique_patterns(&d.output))
            .sum();

        assert_eq!(count_digits_with_unique_number, 26);
    }

    #[test]
    fn test_part2_example() {
        let example = DigitDisplay::from_str(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        )
        .unwrap();

        let decoded = example.decode().unwrap();

        assert_eq!(decoded, 5353)
    }

    #[test]
    fn test_part2() {
        let displays: Vec<DigitDisplay> = helpers::read_lines_parse("day8.testinput").unwrap();

        let sum: usize = displays.iter().flat_map(|d| d.decode()).sum();

        assert_eq!(sum, 61229)
    }
}
