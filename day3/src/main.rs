use bit_vec::BitVec;
use helpers::{read_lines_parse, AocError, AocResult};
use std::cmp::Ordering;
use std::str::FromStr;

// The diagnostic report (your puzzle input) consists of a list of binary numbers which,
// when decoded properly, can tell you many useful things about the conditions of the submarine.
// The first parameter to check is the power consumption.
//
// You need to use the binary numbers in the diagnostic report to generate two new binary numbers
// (called the gamma rate and the epsilon rate). The power consumption can then be found by
// multiplying the gamma rate by the epsilon rate.

#[derive(Clone, Debug)]
struct Report<'a> {
    size: usize,
    data: &'a [ReportBits],
}

impl<'a> Report<'a> {
    pub fn from_bits(data: &'a [ReportBits]) -> Result<Self, AocError> {
        if let Some(first) = data.first() {
            let size = first.len();
            if data.iter().any(|b| b.len() != size) {
                return Err(AocError::ParseStructError(
                    "uneven size/length of bits".to_string(),
                ));
            }
            Ok(Report { size, data })
        } else {
            Err(AocError::ParseStructError(
                "Reports may not be empty".to_string(),
            ))
        }
    }

    // Each bit in the gamma rate can be determined by finding the most common bit in the
    // corresponding position of all numbers in the diagnostic report.
    // It is not specified what should happen if they are equally common. Zeroes will win as by this
    // implementation
    pub fn gamma_rate(&self) -> ReportBits {
        // Report bits are guaranteed to have an uneven number of bits, such that there will always
        // be majority or minority
        let mut result_bits = BitVec::from_elem(self.size, false);
        for position in 0..self.size {
            if let Some(true) = most_common_bit(self.data, position) {
                result_bits.set(position, true)
            }
        }

        ReportBits(result_bits)
    }

    // The epsilon rate is calculated in a similar way [to the gamma rate];
    // rather than use the most common bit, the least common bit from each position is used.
    pub fn epsilon_rate(&self) -> ReportBits {
        let mut gamma_rate = self.gamma_rate();
        gamma_rate.0.negate();

        gamma_rate
    }

    // PART 2
    // Before searching for either rating value, start with the full list of binary numbers from
    // your diagnostic report and consider just the first bit of those numbers. Then:
    //
    //     Keep only numbers selected by the bit criteria for the type of rating value
    //         for which you are searching. Discard numbers which do not match the bit criteria.
    //     If you only have one number left, stop; this is the rating value for which you are
    //         searching.
    //     Otherwise, repeat the process, considering the next bit to the right.

    fn oxygen_generator_rating(&self) -> Option<ReportBits> {
        self.reduce_to_single_rating(true, false)
    }

    // returns Some(ReportBits) if a single ReportBit was left over, otherwise None.
    // TODO get rid of excessive cloning and third parameter
    fn reduce_to_single_rating(
        &self,
        prefer_on_tie: bool,
        invert_common_bit: bool,
    ) -> Option<ReportBits> {
        let mut current: Vec<_> = self.data.to_vec();
        for idx in 0..self.size {
            if current.len() < 2 {
                break;
            }
            // To find oxygen generator rating, determine the most common value (0 or 1)
            // in the current bit position
            // keep only numbers with that bit in that position.
            // If 0 and 1 are equally common, keep values with prefer_on_tie in the position being considered.
            let common_bit = most_common_bit(&current, idx)
                .map(|cb| if invert_common_bit { !cb } else { cb })
                .unwrap_or(prefer_on_tie);

            current = current
                .iter()
                .filter(|b| b.get(idx).unwrap() == common_bit)
                .cloned()
                .collect();
        }

        if current.len() == 1 {
            Some(current[0].clone())
        } else {
            None
        }
    }

    // To find CO2 scrubber rating, determine the least common value (0 or 1) in the current bit
    // position, and keep only numbers with that bit in that position.
    // If 0 and 1 are equally common, keep values with a 0 in the position being considered.
    pub fn co2_scrubber_rating(&self) -> Option<ReportBits> {
        self.reduce_to_single_rating(false, true)
    }
}

#[derive(Clone, Debug)]
struct ReportBits(BitVec);
impl ReportBits {
    fn len(&self) -> usize {
        self.0.len()
    }
    fn get(&self, idx: usize) -> Option<bool> {
        self.0.get(idx)
    }
    pub fn to_decimal(&self) -> u32 {
        let mut sum = 0;
        for i in 0..self.len() {
            if let Some(true) = self.get(i) {
                // starting from the left, the exponent and i are opposite
                // -1 is necessary to account for the index starting at 0 up to n - 1
                // while len() for n elements will be n
                let exponent = self.len() - 1 - i;

                let number = 2_u32.pow(exponent as u32);
                sum += number
            }
        }
        sum
    }
}

// number of ones for the same index in each bitvec
fn count_ones_column(bits: &[ReportBits], idx: usize) -> usize {
    bits.iter()
        .map(|d| d.get(idx))
        .filter(|b| matches!(b, Some(true)))
        .count()
}

// returns Some(true|false) if none is prevalent, None is returned.
fn most_common_bit(bits: &[ReportBits], idx: usize) -> Option<bool> {
    let ones = count_ones_column(bits, idx);
    let zeroes = bits.len() - ones;

    match ones.cmp(&zeroes) {
        Ordering::Less => Some(false),
        Ordering::Equal => None,
        Ordering::Greater => Some(true),
    }
}

// this ignore all chars that are neither 0 nor 1
impl FromStr for ReportBits {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.chars().count();

        let mut vec = BitVec::from_elem(len, false);
        for (n, c) in s.chars().enumerate() {
            match c {
                // all bits are already 0
                '0' => {}
                // set position n to 1
                '1' => vec.set(n, true),
                _ => {
                    return Err(AocError::ParseStructError(format!(
                        "'{}' is not a valid bit",
                        c
                    )))
                }
            }
        }

        Ok(ReportBits(vec))
    }
}

fn main() -> AocResult<()> {
    let input: Vec<ReportBits> = read_lines_parse("day3/day3.input")?;
    let report = Report::from_bits(&input)?;

    let gamma = report.gamma_rate().to_decimal();
    let epsilon = report.epsilon_rate().to_decimal();
    println!(
        "Gamma: {}, Epsilon: {}, Product: {}",
        gamma,
        epsilon,
        gamma * epsilon
    );

    let co2_scrubber_rating = report.co2_scrubber_rating().unwrap().to_decimal();
    let oxygen_generator_rating = report.oxygen_generator_rating().unwrap().to_decimal();
    println!(
        "Co2: {}, oxygen: {}, Product: {}",
        co2_scrubber_rating,
        oxygen_generator_rating,
        co2_scrubber_rating * oxygen_generator_rating
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let input: Vec<ReportBits> = read_lines_parse("day3.testinput").unwrap();
        let report = Report::from_bits(&input).unwrap();

        let gamma_rate = report.gamma_rate();
        let gamma_rate_decimal = gamma_rate.to_decimal();

        let epsilon_rate = report.epsilon_rate();
        let epsilon_rate_decimal = epsilon_rate.to_decimal();

        // So, the gamma rate is the binary number 10110, or 22 in decimal.
        assert!(gamma_rate.0.eq_vec(&[true, false, true, true, false]));
        assert_eq!(gamma_rate_decimal, 22);

        //So, the epsilon rate is 01001, or 9 in decimal.
        assert!(epsilon_rate.0.eq_vec(&[false, true, false, false, true]));
        assert_eq!(epsilon_rate_decimal, 9)
    }

    #[test]
    fn example_part2() {
        let input: Vec<ReportBits> = read_lines_parse("day3.testinput").unwrap();
        let report = Report::from_bits(&input).unwrap();

        // For example, to determine the oxygen generator rating value using the same example
        // diagnostic report from above:
        //
        //     Start with all 12 numbers and consider only the first bit of each number. There are
        //         more 1 bits (7) than 0 bits (5), so keep only the 7 numbers with a 1 in the first
        //         position: 11110, 10110, 10111, 10101, 11100, 10000, and 11001.
        //     Then, consider the second bit of the 7 remaining numbers: there are more 0 bits (4)
        //         than 1 bits (3), so keep only the 4 numbers with a 0 in the second position:
        //         10110, 10111, 10101, and 10000.
        //     In the third position, three of the four numbers have a 1, so keep those three:
        //         10110, 10111, and 10101.
        //     In the fourth position, two of the three numbers have a 1, so keep those two:
        //         10110 and 10111.
        //     In the fifth position, there are an equal number of 0 bits and 1 bits (one each).
        //     So, to find the oxygen generator rating, keep the number with a 1 in that position:
        //         10111.
        //     As there is only one number left, stop; the oxygen generator rating is 10111,
        //         or 23 in decimal.
        //
        let oxygen_generator_rating = report.oxygen_generator_rating().unwrap();
        // Then, to determine the CO2 scrubber rating value from the same example above:
        //
        //     Start again with all 12 numbers and consider only the first bit of each number.
        //         There are fewer 0 bits (5) than 1 bits (7), so keep only the 5 numbers with a 0
        //         in the first position: 00100, 01111, 00111, 00010, and 01010.
        //     Then, consider the second bit of the 5 remaining numbers: there are fewer 1 bits (2)
        //         than 0 bits (3), so keep only the 2 numbers with a 1 in the second position:
        //         01111 and 01010.
        //     In the third position, there are an equal number of 0 bits and 1 bits (one each).
        //         So, to find the CO2 scrubber rating, keep the number with a 0 in that position:
        //         01010.
        //     As there is only one number left, stop; the CO2 scrubber rating is 01010,
        //         or 10 in decimal.
        let co2_scrubber_rating = report.co2_scrubber_rating().unwrap();

        assert_eq!(oxygen_generator_rating.to_decimal(), 23);
        assert!(oxygen_generator_rating
            .0
            .eq_vec(&[true, false, true, true, true]));

        assert_eq!(co2_scrubber_rating.to_decimal(), 10);
        assert!(co2_scrubber_rating
            .0
            .eq_vec(&[false, true, false, true, false]))
    }
}
