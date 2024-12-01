advent_of_code::solution!(2);

struct Input {
    reports: Vec<Report>,
}

impl From<&str> for Input {
    fn from(value: &str) -> Self {
        let reports = value.lines().map(Report::from).collect();
        Self { reports }
    }
}

struct Report(Vec<i32>);

impl Report {
    fn is_safe(&self) -> bool {
        safe_sequence(self.0.as_slice())
    }

    fn is_safe_with_dampening(&self) -> bool {
        if self.is_safe() {
            return true;
        }

        (0..self.0.len()).any(|skip| {
            let mut filtered = self.0.clone();
            filtered.remove(skip);
            safe_sequence(filtered.as_slice())
        })
    }
}

impl From<&str> for Report {
    fn from(value: &str) -> Self {
        let values = value
            .split(' ')
            .map(|n| n.parse())
            .collect::<Result<_, _>>()
            .unwrap();
        Self(values)
    }
}

fn safe_sequence(values: &[i32]) -> bool {
    let diffed_values = values
        .iter()
        .zip(values.iter().skip(1))
        .map(|(n, np1)| np1 - n);

    let mut sign = None;
    for diff in diffed_values {
        let signnum = diff.signum();
        if let Some(s) = sign {
            if signnum != s {
                return false;
            }
        } else {
            sign = Some(signnum);
        }
        if diff.abs() > 3 {
            return false;
        }
    }
    true
}

pub fn part_one(input: &str) -> Option<usize> {
    let Input { reports } = input.into();
    let ans = reports.iter().filter(|r| r.is_safe()).count();
    Some(ans)
}

pub fn part_two(input: &str) -> Option<usize> {
    let Input { reports } = input.into();
    let ans = reports
        .iter()
        .filter(|r| r.is_safe_with_dampening())
        .count();
    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
