use std::fs::File;
use std::io::BufRead;
use std::num::ParseIntError;
use std::path::Path;
use std::{env, io};
use thiserror::Error;

pub type AocResult<T> = Result<T, AocError>;

#[derive(Error, Debug)]
pub enum AocError {
    #[error("io error")]
    IoError(#[from] io::Error),
    #[error("parse int error")]
    ParseIntError(#[from] ParseIntError),
}

pub fn print_current_dir() {
    let path = env::current_dir();
    println!("The current directory is {:?}", path);
}

pub fn read_lines_numbers<P>(filename: P) -> AocResult<Vec<usize>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let mut numbers = Vec::new();
    let reader = io::BufReader::new(file).lines();
    for l in reader {
        let number = l?.parse()?;
        numbers.push(number)
    }

    Ok(numbers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file_numbers() {
        let lines = read_lines_numbers("readline_numbers.input").unwrap();

        assert_eq!(&lines, &[0, 1, 2, 3]);
    }
}
