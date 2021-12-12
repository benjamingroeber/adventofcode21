use helpers::{read_file_string, AocError, AocResult};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
pub struct Edge<'a>(&'a str, &'a str);
const EDGE_DELIMITER: char = '-';
static START_NODE: &str = "start";
static END_NODE: &str = "end";

impl<'a> TryFrom<&'a str> for Edge<'a> {
    type Error = AocError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if let Some((left, right)) = value.split_once(EDGE_DELIMITER) {
            Ok(Edge(left, right))
        } else {
            Err(AocError::ParseStructError(format!(
                "Edge {} doesn't contain the delimiter {}",
                value, EDGE_DELIMITER
            )))
        }
    }
}

#[derive(Debug)]
pub struct Graph<'a> {
    neighbours: HashMap<String, Vec<&'a str>>,
}

impl<'a> Graph<'a> {
    pub fn with_edges(edges: &'a [Edge]) -> Self {
        let mut neighbours = HashMap::new();
        for edge in edges {
            let entry = neighbours
                .entry(edge.0.to_string())
                .or_insert_with(Vec::new);
            if !entry.contains(&edge.1) {
                entry.push(edge.1)
            }
            let entry = neighbours
                .entry(edge.1.to_string())
                .or_insert_with(Vec::new);
            if !entry.contains(&edge.0) {
                entry.push(edge.0)
            }
        }
        Self { neighbours }
    }

    // returns the numbers of distinct paths traversed
    pub fn traverse_visiting_single_caves_once(&self) -> usize {
        self.traverse(START_NODE, HashSet::new(), false)
    }

    // returns the numbers of distinct paths traversed
    pub fn traverse_visiting_single_small_cave_twice(&self) -> usize {
        self.traverse(START_NODE, HashSet::new(), true)
    }

    // big caves can be visited any number of times
    // a single small cave can be visited at most twice
    // and the remaining small caves can be visited at most once
    // However, the caves named start and end can only be visited exactly once each
    fn traverse(
        &'a self,
        node: &'a str,
        mut visited: HashSet<&'a str>,
        allow_visiting_a_small_cave_twice: bool,
    ) -> usize {
        // We reached the end, this counts as a distinct path
        if node == END_NODE {
            1
        } else {
            visited.insert(node);
            let neighbours = self.neighbours.get(node).expect("Node must exist");
            let mut sum = 0;
            for n in neighbours {
                let is_small_cave = n.chars().all(|c| c.is_ascii_lowercase());

                if visited.contains(n) && is_small_cave {
                    if allow_visiting_a_small_cave_twice && n != &START_NODE {
                        sum += self.traverse(n, visited.clone(), false)
                    }
                } else {
                    sum += self.traverse(n, visited.clone(), allow_visiting_a_small_cave_twice)
                }
            }
            sum
        }
    }
}

fn main() -> AocResult<()> {
    let input = read_file_string("day12/day12.input")?;
    let edges: Vec<_> = input
        .lines()
        .map(Edge::try_from)
        .collect::<AocResult<_>>()?;
    let graph = Graph::with_edges(&edges);
    let distinct_paths = graph.traverse_visiting_single_caves_once();
    println!(
        "Distinct paths visiting small caves only once: {}",
        distinct_paths
    );
    let distinct_paths_with_small_cave_twice = graph.traverse_visiting_single_small_cave_twice();
    println!(
        "Distinct paths visiting a single small cave twice: {}",
        distinct_paths_with_small_cave_twice
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1_example1() {
        let input = "start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end";
        let edges: Vec<_> = input
            .lines()
            .map(Edge::try_from)
            .collect::<AocResult<_>>()
            .unwrap();
        let graph = Graph::with_edges(&edges);

        let distinct_paths = graph.traverse_visiting_single_caves_once();
        assert_eq!(distinct_paths, 10);
    }

    #[test]
    fn example_part1_example2() {
        let input =
            "dc-end\nHN-start\nstart-kj\ndc-start\ndc-HN\nLN-dc\nHN-end\nkj-sa\nkj-HN\nkj-dc";
        let edges: Vec<_> = input
            .lines()
            .map(Edge::try_from)
            .collect::<AocResult<_>>()
            .unwrap();
        let graph = Graph::with_edges(&edges);

        let distinct_paths = graph.traverse_visiting_single_caves_once();
        assert_eq!(distinct_paths, 19);
    }

    #[test]
    fn example_part1_example3() {
        let input =
            "fs-end\nhe-DX\nfs-he\nstart-DX\npj-DX\nend-zg\nzg-sl\nzg-pj\npj-he\nRW-he\nfs-DX\npj-RW\nzg-RW\nstart-pj\nhe-WI\nzg-he\npj-fs\nstart-RW";
        let edges: Vec<_> = input
            .lines()
            .map(Edge::try_from)
            .collect::<AocResult<_>>()
            .unwrap();
        let graph = Graph::with_edges(&edges);

        let distinct_paths = graph.traverse_visiting_single_caves_once();
        assert_eq!(distinct_paths, 226);
    }

    #[test]
    fn example_part2_example1() {
        let input = "start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end";
        let edges: Vec<_> = input
            .lines()
            .map(Edge::try_from)
            .collect::<AocResult<_>>()
            .unwrap();
        let graph = Graph::with_edges(&edges);

        let distinct_paths = graph.traverse_visiting_single_small_cave_twice();
        assert_eq!(distinct_paths, 36);
    }

    #[test]
    fn example_part2_example2() {
        let input =
            "dc-end\nHN-start\nstart-kj\ndc-start\ndc-HN\nLN-dc\nHN-end\nkj-sa\nkj-HN\nkj-dc";
        let edges: Vec<_> = input
            .lines()
            .map(Edge::try_from)
            .collect::<AocResult<_>>()
            .unwrap();
        let graph = Graph::with_edges(&edges);

        let distinct_paths = graph.traverse_visiting_single_small_cave_twice();
        assert_eq!(distinct_paths, 103);
    }

    #[test]
    fn example_part2_example3() {
        let input =
            "fs-end\nhe-DX\nfs-he\nstart-DX\npj-DX\nend-zg\nzg-sl\nzg-pj\npj-he\nRW-he\nfs-DX\npj-RW\nzg-RW\nstart-pj\nhe-WI\nzg-he\npj-fs\nstart-RW";
        let edges: Vec<_> = input
            .lines()
            .map(Edge::try_from)
            .collect::<AocResult<_>>()
            .unwrap();
        let graph = Graph::with_edges(&edges);

        let distinct_paths = graph.traverse_visiting_single_small_cave_twice();
        assert_eq!(distinct_paths, 3509);
    }
}
