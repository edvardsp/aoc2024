use std::collections::{HashMap, HashSet};

use rayon::prelude::*;

advent_of_code::solution!(23);

struct Graph<'i> {
    nodes: HashMap<&'i str, HashSet<&'i str>>,
}

impl<'i> Graph<'i> {
    fn trios(&'i self) -> HashSet<[&'i str; 3]> {
        let connections = &self.nodes;
        let mut list = HashSet::new();
        for (&first, conns0) in &self.nodes {
            for &second in conns0 {
                let conns1 = &connections[second];
                for &third in conns0.intersection(conns1) {
                    let mut idx = [first, second, third];
                    idx.sort();
                    list.insert(idx);
                }
            }
        }
        list
    }

    fn largest(&'i self) -> Vec<&'i str> {
        let connections = &self.nodes;
        self.trios()
            .into_par_iter()
            .map(|trio| {
                let mut group: HashSet<_> = HashSet::from_iter(trio);
                for candidate in self.nodes.keys() {
                    if group.contains(candidate) {
                        continue;
                    }
                    if group
                        .iter()
                        .all(|node| connections[node].contains(candidate))
                    {
                        group.insert(candidate);
                    }
                }
                group
            })
            .max_by(|lhs, rhs| lhs.len().cmp(&rhs.len()))
            .map(|group| {
                let mut group = Vec::from_iter(group);
                group.sort();
                group
            })
            .unwrap()
    }
}

impl<'i> From<&'i str> for Graph<'i> {
    fn from(value: &'i str) -> Self {
        let mut nodes: HashMap<_, HashSet<_>> = HashMap::new();
        for line in value.lines() {
            let (lhs, rhs) = line.split_once("-").unwrap();
            nodes.entry(lhs).or_default().insert(rhs);
            nodes.entry(rhs).or_default().insert(lhs);
        }
        Self { nodes }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let graph: Graph = input.into();
    Some(
        graph
            .trios()
            .into_iter()
            .filter(|idx| idx.iter().any(|s| s.starts_with("t")))
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<String> {
    let graph: Graph = input.into();
    Some(graph.largest().join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}
