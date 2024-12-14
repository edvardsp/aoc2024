use regex::Regex;

advent_of_code::solution!(13);

#[derive(Debug)]
struct Game {
    a_button: (isize, isize),
    b_button: (isize, isize),
    prize: (isize, isize),
}

impl Game {
    fn correct_position(&mut self) {
        self.prize.0 += 10000000000000;
        self.prize.1 += 10000000000000;
    }

    fn tokens(&self, max_play: Option<isize>) -> Option<usize> {
        let det = self.a_button.0 * self.b_button.1 - self.a_button.1 * self.b_button.0;
        let det_a = self.prize.0 * self.b_button.1 - self.prize.1 * self.b_button.0;
        let det_b = self.a_button.0 * self.prize.1 - self.a_button.1 * self.prize.0;

        if det_a % det != 0 || det_b % det != 0 {
            return None;
        }

        let a_presses = det_a / det;
        let b_presses = det_b / det;
        if let Some(max_presses) = max_play {
            if a_presses > max_presses || b_presses > max_presses {
                return None;
            }
        }

        let prize = a_presses * 3 + b_presses;
        Some(prize as usize)
    }
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let re = Regex::new(
            r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
        )
        .unwrap();
        let m = re.captures(value).unwrap();
        let a_button = (m[1].parse().unwrap(), m[2].parse().unwrap());
        let b_button = (m[3].parse().unwrap(), m[4].parse().unwrap());
        let prize = (m[5].parse().unwrap(), m[6].parse().unwrap());
        Self {
            a_button,
            b_button,
            prize,
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let ans = input
        .split("\n\n")
        .map(Game::from)
        .filter_map(|game| game.tokens(Some(100)))
        .sum();
    Some(ans)
}

pub fn part_two(input: &str) -> Option<usize> {
    let ans = input
        .split("\n\n")
        .map(Game::from)
        .filter_map(|mut game| {
            game.correct_position();
            game.tokens(None)
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
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
