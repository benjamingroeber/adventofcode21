use helpers::{read_file_string, AocError, AocResult};
use itertools::{Itertools, MinMaxResult};
use std::collections::HashMap;

fn main() -> AocResult<()> {
    let input = read_file_string("day14/day14.input")?;
    if let Some((template, rules)) = input.split_once("\n\n") {
        let rules = parse_rules(rules)?;

        // Part 1
        //Apply 10 steps of pair insertion to the polymer template and find the most and least
        // common elements in the result. What do you get if you take the quantity of the most
        // common element and subtract the quantity of the least common element?
        let inserter = NaivePairInserter::new(rules.clone());
        let mut next = template.to_string();
        for _ in 0..10 {
            next = inserter.pair_insert(&next)?;
        }
        if let MinMaxResult::MinMax((_, min), (_, max)) = NaivePairInserter::count_elements(&next)
            .iter()
            .minmax_by_key(|c| c.1)
        {
            println!("Max minus min: {}", max - min);
        } else {
            println!("Not enough elements");
        }

        // Part 2
        // Apply 40 steps of pair insertion to the polymer template and find the most and least
        // common elements in the result. What do you get if you take the quantity of the most
        // common element and subtract the quantity of the least common element?
        if let Some(mut stateful_inserter) = StatefulPairInserter::new(rules, template) {
            for _ in 0..40 {
                // let start = Instant::now();
                stateful_inserter.step()?;
                // println!("i: {} took {:?}", i, start.elapsed());
            }
            if let MinMaxResult::MinMax((_, min), (_, max)) = stateful_inserter
                .count_elements()
                .iter()
                .minmax_by_key(|c| c.1)
            {
                println!("Max minus min: {}", max - min);
            } else {
                println!("Not enough elements");
            }
        }
    }

    Ok(())
}

type InsertionRules = HashMap<(char, char), char>;
static PAIR_DELIM: &str = " -> ";
fn parse_rules(input: &str) -> AocResult<InsertionRules> {
    let mut rules = InsertionRules::new();
    for line in input.lines().map(|l| l.split_once(PAIR_DELIM)) {
        match line {
            Some((input, output)) if input.len() == 2 && output.len() == 1 => {
                let mut in_chars = input.chars();
                let left = in_chars.next().unwrap();
                let right = in_chars.next().unwrap();
                let output = output.chars().next().unwrap();
                rules.insert((left, right), output);
            }
            None => {
                return Err(AocError::ParseStructError(format!(
                    "Line '{}' not recognized as Rule 2 input and 1 output chars ",
                    input
                )))
            }
            _ => {
                return Err(AocError::ParseStructError(format!(
                    "Line '{}' not recognized as Rule with delimiter '{}' ",
                    input, PAIR_DELIM
                )))
            }
        }
    }
    Ok(rules)
}

struct NaivePairInserter {
    rules: InsertionRules,
}
impl NaivePairInserter {
    fn new(rules: InsertionRules) -> Self {
        Self { rules }
    }
    fn count_elements(s: &str) -> HashMap<char, usize> {
        let mut result = HashMap::new();
        for c in s.chars() {
            let e = result.entry(c).or_insert(0);
            *e += 1
        }
        result
    }
    fn pair_insert_center(&self, input: (char, char)) -> Option<char> {
        self.rules.get(&input).copied()
    }

    pub fn pair_insert(&self, input: &str) -> AocResult<String> {
        let mut result = String::new();
        for tuple in input.chars().tuple_windows() {
            if let Some(center) = self.pair_insert_center(tuple) {
                // append only left and center, as the right will be the next left
                result.push(tuple.0);
                result.push(center);
            } else {
                return Err(AocError::ChallengeError(format!(
                    "No rule found for tuple {:?}",
                    input
                )));
            }
        }
        if let Some(last) = input.chars().last() {
            // append the last character, as there are no later tuples, and it never results as "left"
            result.push(last)
        }
        Ok(result)
    }
}

struct StatefulPairInserter {
    rules: InsertionRules,
    state: HashMap<(char, char), usize>,
    first: char,
}

impl StatefulPairInserter {
    // if template is empty, None will be returned
    fn new(rules: InsertionRules, template: &str) -> Option<Self> {
        template.chars().next().map(|first| {
            let mut state = HashMap::new();
            for tuple in template.chars().tuple_windows() {
                state.entry(tuple).and_modify(|e| *e += 1).or_insert(1);
            }
            StatefulPairInserter {
                rules,
                state,
                first,
            }
        })
    }
    fn count_elements(&self) -> HashMap<char, usize> {
        let mut count = HashMap::new();
        // only count each second char, as they are counted doubly otherwise
        for (key, value) in &self.state {
            count
                .entry(key.1)
                .and_modify(|e| *e += value)
                .or_insert(*value);
        }
        // as each second char is counted, the first ever character is never counted before
        count.entry(self.first).and_modify(|e| *e += 1);
        count
    }
    fn step(&mut self) -> AocResult<()> {
        let old = self.state.clone();
        let mut new = HashMap::new();
        for (current, count) in old.iter() {
            let center = *self.rules.get(current).unwrap();
            // add two new tuples
            let left = (current.0, center);
            let right = (center, current.1);

            // Add count of new tuples
            new.entry(left)
                .and_modify(|e| *e += count)
                .or_insert(*count);
            new.entry(right)
                .and_modify(|e| *e += count)
                .or_insert(*count);
        }
        self.state = new;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_RULES: &str = "CH -> B\nHH -> N\nCB -> H\nNH -> C\nHB -> C\nHC -> B\nHN -> C\nNN -> C\nBH -> H\nNC -> B\nNB -> B\nBN -> B\nBB -> N\nBC -> B\nCC -> N\nCN -> C";

    #[test]
    fn example_part1() {
        let input = "NNCB";
        let rules = parse_rules(TEST_RULES).unwrap();
        let inserter = NaivePairInserter::new(rules);

        let once = inserter.pair_insert(input).unwrap();
        let twice = inserter.pair_insert(&once).unwrap();
        let thrice = inserter.pair_insert(&twice).unwrap();
        let quadrice = inserter.pair_insert(&thrice).unwrap();
        let quintice = inserter.pair_insert(&quadrice).unwrap();
        let quintice_count: usize = NaivePairInserter::count_elements(&quintice).values().sum();

        let mut next = quintice;
        for _ in 5..10 {
            next = inserter.pair_insert(&next).unwrap();
        }
        let ten_count = NaivePairInserter::count_elements(&next);
        let most_common = ten_count.iter().max_by_key(|c| *c.1).unwrap();
        let least_common = ten_count.iter().min_by_key(|c| *c.1).unwrap();

        assert_eq!("NCNBCHB", &once);
        assert_eq!("NBCCNBBBCBHCB", &twice);
        assert_eq!("NBBBCNCCNBBNBNBBCHBHHBCHB", &thrice);
        assert_eq!(
            "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB",
            &quadrice
        );
        assert_eq!(quintice_count, 97);
        assert_eq!(*ten_count.get(&'B').unwrap(), 1749);
        assert_eq!(*ten_count.get(&'C').unwrap(), 298);
        assert_eq!(*ten_count.get(&'H').unwrap(), 161);
        assert_eq!(*ten_count.get(&'N').unwrap(), 865);
        assert_eq!(most_common.1 - least_common.1, 1588);
    }

    #[test]
    fn example_part2() {
        let template = "NNCB";
        let rules = parse_rules(TEST_RULES).unwrap();
        let mut inserter = StatefulPairInserter::new(rules, template).unwrap();

        let never: usize = inserter.count_elements().values().sum();
        inserter.step().unwrap();
        let once: usize = inserter.count_elements().values().sum();
        inserter.step().unwrap();
        let twice: usize = inserter.count_elements().values().sum();
        inserter.step().unwrap();
        let thrice: usize = inserter.count_elements().values().sum();
        inserter.step().unwrap();
        let quadrice: usize = inserter.count_elements().values().sum();
        inserter.step().unwrap();
        let quintice: usize = inserter.count_elements().values().sum();

        for _ in 5..10 {
            inserter.step().unwrap();
        }
        let ten_count = inserter.count_elements();
        let most_common = ten_count.iter().max_by_key(|c| *c.1).unwrap();
        let least_common = ten_count.iter().min_by_key(|c| *c.1).unwrap();

        assert_eq!(never, 4);
        assert_eq!(once, 7);
        assert_eq!(twice, 13);
        assert_eq!(thrice, 25);
        assert_eq!(quadrice, 49);

        assert_eq!(quintice, 97);
        assert_eq!(*ten_count.get(&'B').unwrap(), 1749);
        assert_eq!(*ten_count.get(&'C').unwrap(), 298);
        assert_eq!(*ten_count.get(&'H').unwrap(), 161);
        assert_eq!(*ten_count.get(&'N').unwrap(), 865);
        assert_eq!(most_common.1 - least_common.1, 1588);
    }
}
