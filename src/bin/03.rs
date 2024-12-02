use regex::{Captures, Regex};

advent_of_code::solution!(3);

const RE_OPS: &str = r"mul\((\d+),(\d+)\)";
const RE_OPS2: &str = r"(mul\((\d+),(\d+)\)|do\(\)|don't\(\))";

pub fn part_one(input: &str) -> Option<usize> {
    let re_ops = Regex::new(RE_OPS).unwrap();

    let ans = re_ops.captures_iter(input).map(|c| mul_op(c, 1, 2)).sum();
    Some(ans)
}

pub fn part_two(input: &str) -> Option<usize> {
    let re_ops = Regex::new(RE_OPS2).unwrap();

    let mut sum = 0;
    let mut enable = true;
    for c in re_ops.captures_iter(input) {
        let token = c.get(0).unwrap().as_str();
        if token.starts_with("don't") {
            enable = false;
        } else if token.starts_with("do") {
            enable = true;
        } else if token.starts_with("mul") {
            if enable {
                sum += mul_op(c, 2, 3);
            }
        } else {
            unreachable!("Invalid token {}", token)
        }
    }

    Some(sum)
}

fn mul_op(c: Captures<'_>, i1: usize, i2: usize) -> usize {
    let n1: usize = c.get(i1).unwrap().as_str().parse().unwrap();
    let n2: usize = c.get(i2).unwrap().as_str().parse().unwrap();
    n1 * n2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
