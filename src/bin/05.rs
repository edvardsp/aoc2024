use std::{cmp::Ordering, collections::HashSet};

advent_of_code::solution!(5);

#[derive(Clone)]
struct Update {
    pages: Vec<u32>,
}

impl Update {
    fn is_sorted(&self, rules: &HashSet<(u32, u32)>) -> bool {
        self.pages
            .is_sorted_by(|&lhs, &rhs| rules.contains(&(lhs, rhs)) || !rules.contains(&(rhs, lhs)))
    }

    fn sort(&mut self, rules: &HashSet<(u32, u32)>) {
        self.pages.sort_by(|&lhs, &rhs| {
            if rules.contains(&(lhs, rhs)) {
                Ordering::Less
            } else if rules.contains(&(rhs, lhs)) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });
    }

    fn middle(&self) -> u32 {
        self.pages[self.pages.len() / 2]
    }
}

impl From<&str> for Update {
    fn from(value: &str) -> Self {
        let pages = value.split(',').map(|n| n.parse().unwrap()).collect();
        Self { pages }
    }
}

struct Protocol {
    rules: HashSet<(u32, u32)>,
    updates: Vec<Update>,
}

impl From<&str> for Protocol {
    fn from(value: &str) -> Self {
        let (rules_str, updates_str) = value.split_once("\n\n").unwrap();
        let rules = rules_str
            .lines()
            .map(|line| {
                let (l, r) = line.split_once('|').unwrap();
                (l.parse().unwrap(), r.parse().unwrap())
            })
            .collect();
        let updates = updates_str.lines().map(Update::from).collect();
        Self { rules, updates }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let protocol: Protocol = input.into();
    let ans = protocol
        .updates
        .iter()
        .filter_map(|update| {
            if update.is_sorted(&protocol.rules) {
                Some(update.middle())
            } else {
                None
            }
        })
        .sum();
    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let protocol: Protocol = input.into();
    let ans = protocol
        .updates
        .iter()
        .filter(|update| !update.is_sorted(&protocol.rules))
        .cloned()
        .map(|mut update| {
            update.sort(&protocol.rules);
            update.middle()
        })
        .sum();
    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
