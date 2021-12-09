use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::mem::swap;
use std::num::ParseIntError;
use std::path::Path;
use std::str::FromStr;
use std::{env, io};
use thiserror::Error;

pub type AocResult<T> = Result<T, AocError>;

#[derive(Error, Debug)]
pub enum AocError {
    #[error("io error")]
    IoError(#[from] io::Error),
    #[error("parse int error")]
    ParseIntError(#[from] ParseIntError),
    #[error("parse struct error {0}")]
    ParseStructError(String),
    #[error("grid error {0}")]
    GridError(String),
}

pub fn print_current_dir() {
    let path = env::current_dir();
    println!("The current directory is {:?}", path);
}

pub fn read_file_reader<P>(filename: P) -> AocResult<BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file))
}

pub fn read_file_string<P>(filename: P) -> AocResult<String>
where
    P: AsRef<Path>,
{
    let mut s = String::new();
    let mut reader = read_file_reader(filename)?;
    reader.read_to_string(&mut s)?;
    Ok(s)
}

pub fn read_lines_parse<T, P>(filename: P) -> AocResult<Vec<T>>
where
    P: AsRef<Path>,
    T: FromStr,
    AocError: From<<T as FromStr>::Err>,
{
    let reader = read_file_reader(filename)?;
    let mut parsed = Vec::new();
    for line in reader.lines() {
        let value = line?.parse()?;
        parsed.push(value);
    }

    Ok(parsed)
}

pub struct Grid<T> {
    num_columns: usize,
    data: Vec<T>,
}

// TODO Maybe reimplement Hash only on x and y?
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Point<'a, T> {
    pub x: usize,
    pub y: usize,
    pub value: &'a T,
}

impl<T> Debug for Grid<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, p) in self.data.iter().enumerate() {
            if i != 0 && i % self.num_columns == 0 {
                writeln!(f)?;
            }
            write!(f, "{:?} ", p)?;
        }
        Ok(())
    }
}

impl<T> Grid<T> {
    pub fn new(num_columns: usize) -> Self {
        Grid {
            num_columns,
            data: Vec::new(),
        }
    }
}

impl<T: Default + Clone> Grid<T> {
    pub fn default(x: usize, y: usize) -> Self {
        Grid {
            num_columns: x,
            data: vec![T::default(); x * y],
        }
    }
}

impl<T: Clone> Grid<T> {
    pub fn with_default(x: usize, y: usize, default: T) -> Self {
        Grid {
            num_columns: x,
            data: vec![default; x * y],
        }
    }

    pub fn from_slice(data: &[T], num_columns: usize) -> AocResult<Self> {
        if data.len() % num_columns != 0 {
            return Err(AocError::GridError(format!(
                "Can't divide {} elements in {} columns",
                data.len(),
                num_columns
            )));
        }

        Ok(Grid {
            num_columns,
            data: data.to_vec(),
        })
    }

    pub fn from_first_row(data: &[T]) -> Self {
        let num_columns = data.len();
        Self {
            num_columns,
            data: data.to_vec(),
        }
    }
}

impl<T> Grid<T>
where
    T: ToOwned + ToOwned<Owned = T>,
{
    pub fn add_row(&mut self, data: &[T]) -> AocResult<()> {
        if data.len() != self.num_columns {
            Err(AocError::GridError(format!(
                "Added rows must have the same number of columns as existing rows {}, got {}",
                self.num_columns,
                data.len()
            )))
        } else {
            for i in data.iter() {
                self.data.push(i.to_owned());
            }

            Ok(())
        }
    }
}

impl<T> Grid<T> {
    fn idx(&self, x: usize, y: usize) -> usize {
        self.num_columns * y + x
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.column_count(), self.row_count())
    }

    pub fn column_count(&self) -> usize {
        self.num_columns
    }

    pub fn row_count(&self) -> usize {
        self.data.len() / self.num_columns
    }

    pub fn get(&self, x: usize, y: usize) -> Option<Point<T>> {
        if x >= self.column_count() || y >= self.row_count() {
            return None;
        }
        let idx = self.idx(x, y);
        self.data.get(idx).map(|value| Point { x, y, value })
    }

    pub fn set(&mut self, x: usize, y: usize, mut value: T) -> Option<T> {
        let idx = self.idx(x, y);
        self.data.get_mut(idx).map(|element| {
            swap(element, &mut value);
            // return previous element if index is not out of bounds
            value
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut()
    }

    pub fn iter_row(&self, row: usize) -> GridRowIterator<T> {
        GridRowIterator {
            grid: self,
            row,
            i: 0,
        }
    }

    pub fn iter_col(&self, column: usize) -> GridColumnIterator<T> {
        GridColumnIterator {
            grid: self,
            column,
            i: 0,
        }
    }

    pub fn neighbours(&self, x: usize, y: usize) -> [Option<Point<T>>; 4] {
        let left = if x > 0 { self.get(x - 1, y) } else { None };
        let up = if y > 0 { self.get(x, y - 1) } else { None };
        let right = self.get(x + 1, y);
        let down = self.get(x, y + 1);
        [left, up, right, down]
    }
}

pub struct GridRowIterator<'a, T> {
    grid: &'a Grid<T>,
    row: usize,
    i: usize,
}

impl<'a, T> Iterator for GridRowIterator<'a, T> {
    type Item = Point<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.grid.num_columns {
            None
        } else {
            let item = self.grid.get(self.i, self.row);
            self.i += 1;
            item
        }
    }
}

pub struct GridColumnIterator<'a, T> {
    grid: &'a Grid<T>,
    column: usize,
    i: usize,
}

impl<'a, T> Iterator for GridColumnIterator<'a, T> {
    type Item = Point<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i > self.grid.data.len() / self.grid.num_columns {
            None
        } else {
            let item = self.grid.get(self.column, self.i);
            self.i += 1;
            item
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_get() {
        let data = [1, 2, 3, 4];

        let failed_grid = Grid::from_slice(&data, 3);
        let grid = Grid::from_slice(&data, 2).unwrap();

        assert_eq!(
            grid.get(0, 0),
            Some(Point {
                x: 0,
                y: 0,
                value: &1
            })
        );
        assert_eq!(
            grid.get(1, 0),
            Some(Point {
                x: 1,
                y: 0,
                value: &2
            })
        );
        assert_eq!(
            grid.get(0, 1),
            Some(Point {
                x: 0,
                y: 1,
                value: &3
            })
        );
        assert_eq!(
            grid.get(1, 1),
            Some(Point {
                x: 1,
                y: 1,
                value: &4
            })
        );

        assert_eq!(grid.get(2, 0), None);
        assert_eq!(grid.get(0, 2), None);

        assert!(failed_grid.is_err())
    }

    #[test]
    fn grid_add_row() {
        let data = [1, 2, 3, 4];
        let mut grid = Grid::from_slice(&data, 2).unwrap();

        let get_before1 = grid.get(0, 2).map(|p| *p.value);
        let get_before2 = grid.get(1, 2).map(|p| *p.value);
        let failed_append1 = grid.add_row(&[1, 2, 3]);
        let failed_append2 = grid.add_row(&[1]);
        let failed_append3 = grid.add_row(&[]);
        grid.add_row(&[5, 6]).unwrap();
        let get_after1 = grid.get(0, 2);
        let get_after2 = grid.get(1, 2);

        assert!(failed_append1.is_err());
        assert!(failed_append2.is_err());
        assert!(failed_append3.is_err());
        assert_eq!(get_before1, None);
        assert_eq!(get_before2, None);
        assert_eq!(*get_after1.unwrap().value, 5);
        assert_eq!(*get_after2.unwrap().value, 6);
    }

    #[test]
    fn grid_set() {
        let mut grid = Grid::with_default(3, 3, 0_u32);

        let old_value1 = grid.set(0, 0, 1);
        let old_value2 = grid.set(1, 1, 2);
        let old_value3 = grid.set(2, 2, 3);

        let old_value4 = grid.set(1, 1, 100);
        grid.set(2, 0, 42);
        grid.set(0, 2, 23);

        assert_eq!(old_value1, Some(0));
        assert_eq!(old_value2, Some(0));
        assert_eq!(old_value3, Some(0));
        assert_eq!(old_value4, Some(2));
        assert_eq!(grid.data, [1, 0, 42, 0, 100, 0, 23, 0, 3])
    }

    #[test]
    fn test_row_iterator() {
        let grid = Grid::from_slice(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 3).unwrap();

        let mut first_row = grid.iter_row(0);
        let mut second_row = grid.iter_row(1);
        let mut third_row = grid.iter_row(2);

        assert_eq!(first_row.next().map(|p| p.value), Some(&1));
        assert_eq!(first_row.next().map(|p| p.value), Some(&2));
        assert_eq!(first_row.next().map(|p| p.value), Some(&3));
        assert_eq!(first_row.next(), None);
        assert_eq!(first_row.next(), None);

        assert_eq!(second_row.next().map(|p| p.value), Some(&4));
        assert_eq!(second_row.next().map(|p| p.value), Some(&5));
        assert_eq!(second_row.next().map(|p| p.value), Some(&6));
        assert_eq!(second_row.next(), None);

        assert_eq!(third_row.next().map(|p| p.value), Some(&7));
        assert_eq!(third_row.next().map(|p| p.value), Some(&8));
        assert_eq!(third_row.next().map(|p| p.value), Some(&9));
        assert_eq!(third_row.next(), None);
    }

    #[test]
    fn test_col_iterator() {
        let grid = Grid::from_slice(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 3).unwrap();

        let mut first_col = grid.iter_col(0);
        let mut second_col = grid.iter_col(1);
        let mut third_col = grid.iter_col(2);

        assert_eq!(first_col.next().map(|p| p.value), Some(&1));
        assert_eq!(first_col.next().map(|p| p.value), Some(&4));
        assert_eq!(first_col.next().map(|p| p.value), Some(&7));
        assert_eq!(first_col.next(), None);
        assert_eq!(first_col.next(), None);

        assert_eq!(second_col.next().map(|p| p.value), Some(&2));
        assert_eq!(second_col.next().map(|p| p.value), Some(&5));
        assert_eq!(second_col.next().map(|p| p.value), Some(&8));
        assert_eq!(second_col.next(), None);

        assert_eq!(third_col.next().map(|p| p.value), Some(&3));
        assert_eq!(third_col.next().map(|p| p.value), Some(&6));
        assert_eq!(third_col.next().map(|p| p.value), Some(&9));
        assert_eq!(third_col.next(), None);
    }

    #[test]
    fn test_read_file_numbers() {
        let lines: Vec<usize> = read_lines_parse("readline_numbers.input").unwrap();

        assert_eq!(&lines, &[0, 1, 2, 3]);
    }
}
