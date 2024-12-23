use std::collections::HashMap;

advent_of_code::solution!(22);

struct SecretNumberIter {
    number: usize,
}

impl Iterator for SecretNumberIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.number = ((self.number * 64) ^ self.number) % 16777216;
        self.number = ((self.number / 32) ^ self.number) % 16777216;
        self.number = ((self.number * 2048) ^ self.number) % 16777216;
        Some(self.number)
    }
}

fn generate(number: usize) -> SecretNumberIter {
    SecretNumberIter { number }
}

pub fn part_one(input: &str) -> Option<usize> {
    let ans = input
        .lines()
        .map(|line| line.parse().unwrap())
        .filter_map(|n| generate(n).take(2000).last())
        .sum();
    Some(ans)
}

pub fn part_two(input: &str) -> Option<usize> {
    let numbers: Vec<_> = input.lines().map(|line| line.parse().unwrap()).collect();

    let mut tally = HashMap::new();
    for number in numbers {
        let mut cache = HashMap::new();
        let bananas: Vec<_> = generate(number)
            .take(2000)
            .map(|n| (n % 10) as i32)
            .collect();
        for window in bananas.windows(5) {
            let sequence: [i32; 4] = std::array::from_fn(|i| window[i + 1] - window[i]);
            let value = window[4];
            cache.entry(sequence).or_insert(value);
        }

        for (sequence, value) in cache {
            tally
                .entry(sequence)
                .and_modify(|e| *e += value)
                .or_insert(value);
        }
    }

    tally.values().max().map(|&n| n as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(23));
    }
}
