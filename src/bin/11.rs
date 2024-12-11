use std::collections::HashMap;

advent_of_code::solution!(11);

fn split_digits(value: usize) -> Option<(usize, usize)> {
    let log = (value as f64 + 0.5).log10().ceil() as u32;
    if log % 2 == 0 {
        let separator = 10_usize.pow(log / 2);
        let upper = value / separator;
        let lower = value % separator;
        Some((upper, lower))
    } else {
        None
    }
}

fn blink(memo: &mut HashMap<(usize, usize), usize>, stone: usize, step: usize) -> usize {
    let state = (stone, step);
    if let Some(total) = memo.get(&state) {
        return *total;
    }

    let total = if step == 0 {
        1
    } else if stone == 0 {
        blink(memo, 1, step - 1)
    } else if let Some((upper, lower)) = split_digits(stone) {
        blink(memo, upper, step - 1) + blink(memo, lower, step - 1)
    } else {
        blink(memo, stone * 2024, step - 1)
    };

    memo.insert(state, total);
    total
}

fn total_blinking(stones: &[usize], steps: usize) -> usize {
    let mut memo = HashMap::new();
    stones
        .iter()
        .map(|stone| blink(&mut memo, *stone, steps))
        .sum()
}

pub fn part_one(input: &str) -> Option<usize> {
    let stones: Vec<usize> = input.split(' ').map(|n| n.parse().unwrap()).collect();
    Some(total_blinking(&stones, 25))
}

pub fn part_two(input: &str) -> Option<usize> {
    let stones: Vec<usize> = input.split(' ').map(|n| n.parse().unwrap()).collect();
    Some(total_blinking(&stones, 75))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
