use regex::Regex;

advent_of_code::solution!(3);

const RE_OPS: &str = r"mul\((\d+),(\d+)\)";
const RE_OPS2: &str = r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)";

pub fn part_one(input: &str) -> Option<usize> {
    let re_ops = Regex::new(RE_OPS).unwrap();

    let ans = re_ops
        .captures_iter(input)
        .map(|c| mul_op(&c[1], &c[2]))
        .sum();
    Some(ans)
}

pub fn part_two(input: &str) -> Option<usize> {
    let re_ops = Regex::new(RE_OPS2).unwrap();

    let mut sum = 0;
    let mut enable = true;
    for c in re_ops.captures_iter(input) {
        match get_token(&c[0]) {
            "do" => enable = true,
            "don't" => enable = false,
            "mul" if enable => sum += mul_op(&c[1], &c[2]),
            "mul" => {}
            token => unreachable!("Invalid token {}", token),
        }
    }

    Some(sum)
}

fn get_token(s: &str) -> &str {
    s.find('(').map(|i| &s[..i]).unwrap_or(s)
}

fn mul_op(t1: &str, t2: &str) -> usize {
    let n1: usize = t1.parse().unwrap();
    let n2: usize = t2.parse().unwrap();
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
