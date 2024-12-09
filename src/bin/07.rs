advent_of_code::solution!(7);

fn concat(lhs: usize, rhs: usize) -> usize {
    let pad = (rhs as f64 + 0.5).log10().ceil() as u32;
    lhs * 10_usize.pow(pad) + rhs
}

fn calculate(target: usize, lhs: usize, values: &[usize], ops: &[Operator]) -> bool {
    if values.is_empty() {
        return target == lhs;
    }
    if lhs > target {
        return false;
    }
    let (&rhs, values) = values.split_first().unwrap();
    ops.iter().any(|op| match op {
        Operator::Plus => calculate(target, lhs + rhs, values, ops),
        Operator::Multiply => calculate(target, lhs * rhs, values, ops),
        Operator::Concat => calculate(target, concat(lhs, rhs), values, ops),
    })
}

#[derive(Copy, Clone, Debug)]
enum Operator {
    Plus,
    Multiply,
    Concat,
}

struct Equation {
    value: usize,
    tokens: Vec<usize>,
}

impl Equation {
    fn calibrate(&self) -> Option<usize> {
        const OPS: &[Operator] = &[Operator::Plus, Operator::Multiply];
        if calculate(self.value, self.tokens[0], &self.tokens[1..], OPS) {
            Some(self.value)
        } else {
            None
        }
    }

    fn calibrate_with_concat(&self) -> Option<usize> {
        const OPS: &[Operator] = &[Operator::Plus, Operator::Multiply, Operator::Concat];
        if calculate(self.value, self.tokens[0], &self.tokens[1..], OPS) {
            Some(self.value)
        } else {
            None
        }
    }
}

impl From<&str> for Equation {
    fn from(value: &str) -> Self {
        let (value_str, tokens_str) = value.split_once(':').unwrap();
        let value = value_str.parse().unwrap();
        let tokens = tokens_str
            .trim()
            .split(' ')
            .map(|n| n.parse().unwrap())
            .collect();
        Self { value, tokens }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let equations: Vec<_> = input.lines().map(Equation::from).collect();
    let ans = equations.iter().filter_map(|eq| eq.calibrate()).sum();
    // panic!("awdwa");
    Some(ans)
}

pub fn part_two(input: &str) -> Option<usize> {
    let equations: Vec<_> = input.lines().map(Equation::from).collect();
    let ans = equations
        .iter()
        .filter_map(|eq| eq.calibrate_with_concat())
        .sum();

    // panic!("awdwa");
    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
