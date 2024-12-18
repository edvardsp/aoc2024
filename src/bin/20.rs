use std::collections::{HashSet, VecDeque};

use ndarray::Array2;

advent_of_code::solution!(20);

type Coord = (usize, usize);

struct Map {
    tiles: Array2<char>,
}

impl Map {
    fn neighbors(&self, coord: &Coord) -> impl Iterator<Item = Coord> {
        let (height, width) = self.tiles.dim();
        let (y, x) = *coord;
        let mut v = Vec::new();
        if y > 0 {
            v.push((y - 1, x));
        }
        if y + 1 < height {
            v.push((y + 1, x));
        }
        if x > 0 {
            v.push((y, x - 1));
        }
        if x + 1 < width {
            v.push((y, x + 1));
        }
        v.into_iter()
    }

    fn find_tile(&self, ch: char) -> Option<Coord> {
        for (coord, tile) in self.tiles.indexed_iter() {
            if *tile == ch {
                return Some(coord);
            }
        }
        None
    }

    fn start(&self) -> Coord {
        self.find_tile('S').expect("Start tile must exist")
    }

    fn end(&self) -> Coord {
        self.find_tile('E').expect("End tile must exist")
    }

    fn is_wall(&self, pos: &Coord) -> bool {
        self.tiles[*pos] == '#'
    }

    fn cheats(&self, cheat_duration: usize, cheat_threshold: usize) -> usize {
        let start = self.start();
        let end = self.end();

        let mut path = Vec::new();
        let mut visited = HashSet::new();

        let mut queue = VecDeque::new();
        queue.push_back(start);
        while let Some(pos) = queue.pop_front() {
            path.push(pos);
            visited.insert(pos);
            if pos == end {
                break;
            }
            for next in self.neighbors(&pos) {
                if self.is_wall(&next) || visited.contains(&next) {
                    continue;
                }
                queue.push_back(next);
            }
        }

        let mut total_cheats = 0;
        for (i, begin) in path.iter().enumerate() {
            for (j, end) in path.iter().enumerate().skip(i) {
                let cheat_cost = begin.0.abs_diff(end.0) + begin.1.abs_diff(end.1);
                if cheat_cost > cheat_duration {
                    continue;
                }
                let potential_savings = j - i;
                if cheat_cost < potential_savings {
                    let cheat_value = potential_savings - cheat_cost;
                    if cheat_value >= cheat_threshold {
                        total_cheats += 1;
                    }
                }
            }
        }

        total_cheats
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let chars: Vec<_> = value.chars().filter(|c| *c != '\n').collect();
        let height = value.lines().count();
        let width = chars.len() / height;
        let shape = (height, width);
        let tiles = Array2::from_shape_vec(shape, chars).unwrap();
        Self { tiles }
    }
}

fn get_cheats(input: &str, cheat_duration: usize, cheat_threshold: usize) -> usize {
    let map: Map = input.into();
    map.cheats(cheat_duration, cheat_threshold)
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(get_cheats(input, 2, 100))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(get_cheats(input, 20, 100))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = get_cheats(&advent_of_code::template::read_file("examples", DAY), 2, 0);
        assert_eq!(result, 44);
    }

    #[test]
    fn test_part_two() {
        let result = get_cheats(
            &advent_of_code::template::read_file("examples", DAY),
            20,
            50,
        );
        assert_eq!(result, 285);
    }
}
