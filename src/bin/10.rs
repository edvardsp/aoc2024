use std::collections::{HashSet, VecDeque};

use ndarray::Array2;

advent_of_code::solution!(10);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Coord(isize, isize);

impl Coord {
    fn as_sized(&self) -> Option<(usize, usize)> {
        if self.0 < 0 || self.1 < 0 {
            None
        } else {
            Some((self.0 as usize, self.1 as usize))
        }
    }

    fn neighbors(&self) -> [Self; 4] {
        [
            Self(self.0 - 1, self.1),
            Self(self.0 + 1, self.1),
            Self(self.0, self.1 - 1),
            Self(self.0, self.1 + 1),
        ]
    }
}

impl From<(usize, usize)> for Coord {
    fn from(value: (usize, usize)) -> Self {
        Self(value.0 as isize, value.1 as isize)
    }
}

struct Map {
    tiles: Array2<u8>,
}

impl Map {
    fn tile(&self, coord: Coord) -> Option<u8> {
        self.tiles.get(coord.as_sized()?).copied()
    }

    fn trailhead_score(&self, coord: Coord) -> (usize, usize) {
        let mut peaks = HashSet::new();
        let mut summa = 0;
        let mut queue = VecDeque::new();
        queue.push_back(coord);
        while let Some(coord) = queue.pop_front() {
            let Some(current_tile) = self.tile(coord) else {
                continue;
            };
            if current_tile == 9 {
                peaks.insert(coord);
                summa += 1;
                continue;
            }
            for neighbor in coord.neighbors() {
                let Some(next_tile) = self.tile(neighbor) else {
                    continue;
                };
                if next_tile == current_tile + 1 {
                    queue.push_back(neighbor);
                }
            }
        }
        (peaks.len(), summa)
    }

    fn score(&self) -> usize {
        self.tiles
            .indexed_iter()
            .filter_map(|(coord, &tile)| if tile == 0 { Some(coord.into()) } else { None })
            .map(|coord| self.trailhead_score(coord).0)
            .sum()
    }

    fn distinct_score(&self) -> usize {
        self.tiles
            .indexed_iter()
            .filter_map(|(coord, &tile)| if tile == 0 { Some(coord.into()) } else { None })
            .map(|coord| self.trailhead_score(coord).1)
            .sum()
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let chars: Vec<_> = value
            .lines()
            .flat_map(|line| line.as_bytes())
            .map(|&b| if b.is_ascii_digit() { b - b'0' } else { b'.' })
            .collect();
        let height = value.lines().count();
        let width = chars.len() / height;
        let shape = (height, width);
        let tiles = Array2::from_shape_vec(shape, chars).unwrap();
        Self { tiles }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let map: Map = input.into();
    Some(map.score())
}

pub fn part_two(input: &str) -> Option<usize> {
    let map: Map = input.into();
    Some(map.distinct_score())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
