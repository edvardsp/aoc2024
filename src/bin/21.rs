use std::{cmp::Ordering, collections::HashMap};

advent_of_code::solution!(21);

fn numpad_coord(button: char) -> (usize, usize) {
    match button {
        '7' => (0, 0),
        '8' => (1, 0),
        '9' => (2, 0),
        '4' => (0, 1),
        '5' => (1, 1),
        '6' => (2, 1),
        '1' => (0, 2),
        '2' => (1, 2),
        '3' => (2, 2),
        '0' => (1, 3),
        'A' => (2, 3),
        _ => unreachable!("Invalid numpad button: {}", button),
    }
}

fn press_numpad(input: &str) -> String {
    let mut sequence = String::new();
    let mut current = numpad_coord('A');
    for button in input.chars().map(numpad_coord) {
        if current != button {
            let (sx, sy) = current;
            let (dx, dy) = button;

            let nx = sx.abs_diff(dx);
            let ny = sy.abs_diff(dy);

            let mut horizontal = match sx.cmp(&dx) {
                Ordering::Equal => vec![],
                Ordering::Less => vec!['>'; nx],
                Ordering::Greater => vec!['<'; nx],
            };

            let mut vertical = match sy.cmp(&dy) {
                Ordering::Equal => vec![],
                Ordering::Less => vec!['v'; ny],
                Ordering::Greater => vec!['^'; ny],
            };

            let mut presses = Vec::new();
            match (sx.cmp(&dx), sy.cmp(&dy)) {
                (Ordering::Equal, Ordering::Equal) => {}
                (_, Ordering::Equal) => presses.append(&mut horizontal),
                (Ordering::Equal, _) => presses.append(&mut vertical),
                (Ordering::Greater, Ordering::Greater) => {
                    if sy == 3 && dx == 0 {
                        presses.append(&mut vertical);
                        presses.append(&mut horizontal);
                    } else {
                        presses.append(&mut horizontal);
                        presses.append(&mut vertical);
                    }
                }
                (Ordering::Less, Ordering::Less) => {
                    if sx == 0 && dy == 3 {
                        presses.append(&mut horizontal);
                        presses.append(&mut vertical);
                    } else {
                        presses.append(&mut vertical);
                        presses.append(&mut horizontal);
                    }
                }
                (Ordering::Greater, Ordering::Less) => {
                    presses.append(&mut vertical);
                    presses.append(&mut horizontal);
                }
                (Ordering::Less, Ordering::Greater) => {
                    presses.append(&mut vertical);
                    presses.append(&mut horizontal);
                }
            }
            sequence.extend(presses.into_iter());
        }
        sequence.push('A');
        current = button;
    }
    sequence
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct State {
    input: String,
    depth: usize,
}

type Cache = HashMap<State, usize>;

fn press_keypad(input: &str, depth: usize, cache: &mut Cache) -> usize {
    if depth == 0 {
        return input.len();
    }

    let state = State {
        input: input.to_string(),
        depth,
    };

    if let Some(presses) = cache.get(&state) {
        return *presses;
    }

    let mut total_presses = 0;
    let mut current = 'A';
    for button in input.chars() {
        let keys = match (current, button) {
            ('A', 'A') => "A",
            ('A', '^') => "<A",
            ('A', '>') => "vA",
            ('A', 'v') => "<vA",
            ('A', '<') => "v<<A",

            ('^', 'A') => ">A",
            ('^', '^') => "A",
            ('^', '>') => "v>A",
            ('^', 'v') => "vA",
            ('^', '<') => "v<A",

            ('>', 'A') => "^A",
            ('>', '^') => "<^A",
            ('>', '>') => "A",
            ('>', 'v') => "<A",
            ('>', '<') => "<<A",

            ('v', 'A') => "^>A",
            ('v', '^') => "^A",
            ('v', '>') => ">A",
            ('v', 'v') => "A",
            ('v', '<') => "<A",

            ('<', 'A') => ">>^A",
            ('<', '^') => ">^A",
            ('<', '>') => ">>A",
            ('<', 'v') => ">A",
            ('<', '<') => "A",
            _ => unreachable!("Invalid combination"),
        };

        total_presses += press_keypad(keys, depth - 1, cache);
        current = button;
    }

    cache.insert(state, total_presses);
    total_presses
}

fn sequence_len(input: &str, chain: usize) -> usize {
    let sequence = press_numpad(input);
    let mut cache = Cache::new();
    press_keypad(&sequence, chain, &mut cache)
}

fn numeric_value(input: &str) -> usize {
    input.strip_suffix("A").unwrap().parse().unwrap()
}

fn calculate(input: &str, chain: usize) -> usize {
    sequence_len(input, chain) * numeric_value(input)
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(input.lines().map(|line| calculate(line, 2)).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(input.lines().map(|line| calculate(line, 25)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154115708116294));
    }
}
