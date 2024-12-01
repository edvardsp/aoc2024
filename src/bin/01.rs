use std::collections::HashMap;

advent_of_code::solution!(1);

struct Input {
    left: Vec<u32>,
    right: Vec<u32>,
}

impl From<&str> for Input {
    fn from(value: &str) -> Self {
        let (left, right) = value
            .lines()
            .flat_map(|line| line.split_once("   "))
            .map(|(l, r)| (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap()))
            .unzip();
        Self { left, right }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let Input {
        mut left,
        mut right,
    } = Input::from(input);

    left.sort();
    right.sort();

    let ans = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum();
    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let Input { left, right } = Input::from(input);

    let mut count = HashMap::new();
    for n in &right {
        count.entry(n).and_modify(|e| *e += 1).or_insert(1);
    }

    let ans = left.iter().map(|n| n * count.get(n).unwrap_or(&0)).sum();
    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
