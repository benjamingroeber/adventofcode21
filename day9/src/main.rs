use helpers::{read_file_string, AocResult, Grid};
use std::collections::{HashSet, VecDeque};

type Unit = u32;
type Point<'a> = helpers::Point<'a, Unit>;
const BASIN_DELIMITER: Unit = 9;

#[derive(Debug)]
struct SmokeBasin {
    data: Grid<Unit>,
}

impl SmokeBasin {
    fn from_input(data: &str) -> AocResult<Self> {
        let mut numbers = Vec::new();
        for row in data.lines() {
            let nums: Vec<_> = row.chars().filter_map(|c| c.to_digit(10)).collect();
            numbers.push(nums);
        }

        let mut rows = numbers.iter();
        let first_row = rows.next().unwrap();

        let mut data = Grid::from_first_row(first_row);
        for r in rows {
            data.add_row(r)?;
        }

        Ok(Self { data })
    }

    fn is_low_point(&self, point: &Point) -> bool {
        if *point.value == BASIN_DELIMITER {
            return false;
        }
        self.data
            .neighbours(point.x, point.y)
            .into_iter()
            .flatten()
            .all(|other| point.value < other.value)
    }

    fn get_low_points(&self) -> Vec<Point> {
        let (x_dim, y_dim) = self.data.dimensions();

        let mut low_points = Vec::new();
        for y in 0..y_dim {
            for x in 0..x_dim {
                if let Some(node) = self.data.get(x, y) {
                    if self.is_low_point(&node) {
                        low_points.push(node)
                    }
                }
            }
        }
        low_points
    }

    fn get_basin_size(&self, point: &Point) -> Option<usize> {
        if !self.is_low_point(point) {
            return None;
        }

        let mut queue = VecDeque::from([point.clone()]);
        let mut visited = HashSet::new();
        visited.insert(point.clone());

        while !queue.is_empty() {
            let v_point = queue.pop_front().unwrap();
            for neighbour in self.data.neighbours(v_point.x, v_point.y).iter().flatten() {
                if *neighbour.value < BASIN_DELIMITER && !visited.contains(neighbour) {
                    queue.push_back(neighbour.clone());
                    visited.insert(neighbour.clone());
                }
            }
        }

        Some(visited.len())
    }
}

fn main() -> AocResult<()> {
    let input = read_file_string("day9/day9.input")?;

    let basin = SmokeBasin::from_input(&input)?;
    let low_points = basin.get_low_points();

    // Part 1
    // The risk level of a low point is 1 plus its height
    // What is the sum of the risk levels of all low points on your heightmap?
    let risk_values: Unit = low_points.iter().map(|p| p.value + 1).sum();
    println!("Risk Values: {}", risk_values);

    // Part 2
    let mut basins: Vec<_> = low_points
        .iter()
        .filter_map(|b| basin.get_basin_size(b).map(|s| (b, s)))
        .collect();

    basins.sort_unstable_by_key(|s| s.1);
    basins.reverse();

    println!(
        "Basins: {:?}\nProduct of biggest three: {}",
        basins,
        basins[..3].iter().map(|(_, n)| n).product::<usize>()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let input = read_file_string("day9.testinput").unwrap();

        let basin = SmokeBasin::from_input(&input).unwrap();
        let low_points = basin.get_low_points();

        assert!(low_points.contains(&Point {
            x: 1,
            y: 0,
            value: &1
        }));
        assert!(low_points.contains(&Point {
            x: 9,
            y: 0,
            value: &0
        }));
        assert!(low_points.contains(&Point {
            x: 2,
            y: 2,
            value: &5
        }));
        assert!(low_points.contains(&Point {
            x: 6,
            y: 4,
            value: &5
        }));
        assert_eq!(low_points.len(), 4);
    }

    #[test]
    fn example_part2() {
        let input = read_file_string("day9.testinput").unwrap();

        let basin = SmokeBasin::from_input(&input).unwrap();
        let low_points = basin.get_low_points();

        let basins: Vec<_> = low_points
            .iter()
            .filter_map(|b| basin.get_basin_size(b))
            .collect();

        assert!(basins.contains(&3));
        assert!(basins.contains(&14));
        assert_eq!(basins.iter().filter(|&&n| n == 9).count(), 2)
    }
}
