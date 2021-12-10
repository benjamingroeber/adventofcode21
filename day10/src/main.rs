use helpers::AocResult;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Line<'a> {
    Empty,
    Corrupted(&'a str, OpeningToken, char),
    Incomplete(&'a str, Vec<OpeningToken>),
    Complete(&'a str),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum OpeningToken {
    Round,
    Square,
    Curly,
    Pointy,
}

fn main() -> AocResult<()> {
    let input = helpers::read_file_string("day10/day10.input")?;

    let lines: Vec<_> = input.lines().map(|l| parse_line(l)).collect();

    // Part 1
    let score: usize = lines
        .iter()
        .filter_map(|l| match l {
            Line::Corrupted(_, _, invalid_char) => Some(illegal_points(*invalid_char)),
            _ => None,
        })
        .sum();

    println!("Corruption Score is {}", score);

    // Part 2
    let mut incomplete_scores: Vec<_> = lines
        .iter()
        .filter_map(|l| match l {
            Line::Incomplete(_, open) => Some(autocomplete_score(open)),
            _ => None,
        })
        .collect();
    // Autocomplete tools are an odd bunch: the winner is found by sorting all of the scores and
    // then taking the middle score. (There will always be an odd number of scores to consider.)
    // In this example, the middle score is 288957 because there are the same number of scores
    // smaller and larger than it.
    incomplete_scores.sort_unstable();

    let middle = incomplete_scores.len() / 2;
    println!(
        "Autocomplete center score {} of {} is {}",
        middle,
        incomplete_scores.len(),
        incomplete_scores[middle]
    );

    Ok(())
}

fn closing_char(c: OpeningToken) -> char {
    match c {
        OpeningToken::Round => ')',
        OpeningToken::Square => ']',
        OpeningToken::Curly => '}',
        OpeningToken::Pointy => '>',
    }
}
fn opening_token(c: char) -> Option<OpeningToken> {
    match c {
        '(' => Some(OpeningToken::Round),
        '[' => Some(OpeningToken::Square),
        '{' => Some(OpeningToken::Curly),
        '<' => Some(OpeningToken::Pointy),
        _ => None,
    }
}

fn parse_line(s: &str) -> Line {
    let mut chars = s.chars();
    if let Some(first_char) = chars.next() {
        if let Some(first_token) = opening_token(first_char) {
            // let mut root = Token::new(first_token);
            let mut open: Vec<_> = vec![first_token];
            for c in chars {
                if let Some(opening) = opening_token(c) {
                    open.push(opening)
                } else if let Some(last) = open.last() {
                    if c == closing_char(*last) {
                        open.pop();
                    } else {
                        return Line::Corrupted(s, *last, c);
                    }
                }
            }
            if open.is_empty() {
                return Line::Complete(s);
            } else {
                return Line::Incomplete(s, open);
            }
        }
    }
    Line::Empty
}

// Did you know that syntax checkers actually have contests to see who can get the high score for
// syntax errors in a file? It's true! To calculate the syntax error score for a line, take the
// first illegal character on the line and look it up in the following table:
//
//     ): 3 points.
//     ]: 57 points.
//     }: 1197 points.
//     >: 25137 points.
fn illegal_points(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Invalid illegal character"),
    }
}

// Did you know that autocomplete tools also have contests? It's true!
// The score is determined by considering the completion string character-by-character.
// Start with a total score of 0. Then, for each character, multiply the total score by 5 and then
// increase the total score by the point value given for the character in the following table:
//
//     ): 1 point.
//     ]: 2 points.
//     }: 3 points.
//     >: 4 points.
fn completion_points(t: OpeningToken) -> usize {
    match t {
        OpeningToken::Round => 1,
        OpeningToken::Square => 2,
        OpeningToken::Curly => 3,
        OpeningToken::Pointy => 4,
    }
}

fn autocomplete_score(open: &[OpeningToken]) -> usize {
    // start from the back, as they are the first ones to be closed
    open.iter()
        .rev()
        .fold(0, |acc, x| (acc * 5) + completion_points(*x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_tokens_ok() {
        let ok_tokens = [
            "{}",
            "[]",
            "([])",
            "<([{}])>",
            "(((((((((())))))))))",
            "[<>({}){}[([])<>]]",
        ];

        let results: Vec<_> = ok_tokens.iter().map(|t| parse_line(t)).collect();

        for l in results {
            assert!(matches! {l, Line::Complete(_)})
        }
    }

    #[test]
    fn parse_tokens_corrupted() {
        let corrupted_tokens = [
            "{]",
            "{([(<{}[<>[]}>{[]{[(<()>",
            "[[<[([]))<([[{}[[()]]]",
            "[{[{({}]{}}([{[{{{}}([]",
            "[<(<(<(<{}))><([]([]()",
            "<{([([[(<>()){}]>(<<{{",
        ];

        let results: Vec<_> = corrupted_tokens.iter().map(|t| parse_line(t)).collect();

        for l in results {
            assert!(matches! {l, Line::Corrupted(..)})
        }
    }

    #[test]
    fn parse_tokens_incomplete() {
        let incomplete_tokens = ["{", "{([(<{}[<", "[[<[([])]<([[{"];

        let results: Vec<_> = incomplete_tokens.iter().map(|t| parse_line(t)).collect();

        for l in results {
            assert!(matches! {l, Line::Incomplete(..)})
        }
    }

    static TEST_INPUT: [&str; 10] = [
        "[({(<(())[]>[[{[]{<()<>>",
        "[(()[<>])]({[<{<<[]>>(",
        "{([(<{}[<>[]}>{[]{[(<()>",
        "(((({<>}<{<{<>}{[]{[]{}",
        "[[<[([]))<([[{}[[()]]]",
        "[{[{({}]{}}([{[{{{}}([]",
        "{<[[]]>}<{[{[{[]{()[[[]",
        "[<(<(<(<{}))><([]([]()",
        "<{([([[(<>()){}]>(<<{{",
        "<{([{{}}[<[[[<>{}]]]>[]]",
    ];

    #[test]
    fn example_part1() {
        let results: Vec<_> = TEST_INPUT.iter().map(|t| parse_line(t)).collect();

        let invalid_chars: Vec<_> = results
            .iter()
            .filter_map(|l| match l {
                Line::Corrupted(_, _, invalid_char) => Some(invalid_char),
                _ => None,
            })
            .collect();

        let score: usize = invalid_chars.iter().map(|&&l| illegal_points(l)).sum();

        // In the above example,
        // an illegal ) was found twice (2*3 = 6 points),
        // an illegal ] wasfound once (57 points),
        // an illegal } was found once (1197 points),
        // and an illegal > was found once (25137 points).
        // So, the total syntax error score for this file is 6+57+1197+25137 = 26397 points!
        assert_eq!(invalid_chars.len(), 5);
        assert_eq!(score, 26397);
    }

    #[test]
    fn example_part2() {
        let results: Vec<_> = TEST_INPUT.iter().map(|t| parse_line(t)).collect();

        let incomplete_lines: Vec<_> = results
            .iter()
            .filter_map(|l| match l {
                Line::Incomplete(_, open) => Some(open),
                _ => None,
            })
            .collect();

        let scores: Vec<_> = incomplete_lines
            .iter()
            .map(|l| autocomplete_score(l))
            .collect();

        // In the example above, there are five incomplete lines:
        //
        //     [({(<(())[]>[[{[]{<()<>> - Complete by adding }}]])})].
        //     [(()[<>])]({[<{<<[]>>( - Complete by adding )}>]}).
        //     (((({<>}<{<{<>}{[]{[]{} - Complete by adding }}>}>)))).
        //     {<[[]]>}<{[{[{[]{()[[[] - Complete by adding ]]}}]}]}>.
        //     <{([{{}}[<[[[<>{}]]]>[]] - Complete by adding ])}>.
        assert_eq!(incomplete_lines.len(), 5);
        assert_eq!(scores, [288957, 5566, 1480781, 995444, 294]);
    }
}
