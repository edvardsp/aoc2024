use std::collections::HashMap;

advent_of_code::solution!(19);

struct Input {
    towels: Vec<String>,
    designs: Vec<String>,
}

impl From<&str> for Input {
    fn from(value: &str) -> Self {
        let (towels_str, designs_str) = value.split_once("\n\n").unwrap();
        let towels = towels_str.split(", ").map(str::to_string).collect();
        let designs = designs_str.lines().map(str::to_string).collect();
        Self { towels, designs }
    }
}

fn valid_designs<'m, 's: 'm>(
    memo: &'m mut HashMap<&'s str, usize>,
    towels: &[String],
    design: &'s str,
) -> usize {
    if design.is_empty() {
        return 1;
    }

    if let Some(ans) = memo.get(design) {
        return *ans;
    }

    let ans = towels
        .iter()
        .filter_map(|towel| design.strip_prefix(towel))
        .map(|design2| valid_designs(memo, towels, design2))
        .sum();
    memo.insert(design, ans);
    ans
}

pub fn part_one(input: &str) -> Option<usize> {
    let Input { towels, designs } = input.into();
    let mut memo = HashMap::new();
    let ans = designs
        .iter()
        .filter(|design| valid_designs(&mut memo, &towels, design) != 0)
        .count();
    Some(ans)
}

pub fn part_two(input: &str) -> Option<usize> {
    let Input { towels, designs } = input.into();
    let mut memo = HashMap::new();
    let ans = designs
        .iter()
        .map(|design| valid_designs(&mut memo, &towels, design))
        .sum();
    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
