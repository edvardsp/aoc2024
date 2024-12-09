use std::{collections::HashSet, hash::Hash};

use ndarray::Array2;

advent_of_code::solution!(6);

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotate_right_90(&self) -> Self {
        match *self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
struct Person {
    pos: (usize, usize),
    dir: Direction,
}

impl Person {
    fn ahead(&self, map: &Map) -> Option<(usize, usize)> {
        let (max_y, max_x) = map.shape();
        let (y, x) = self.pos;
        match self.dir {
            Direction::Up if y > 0 => Some((y - 1, x)),
            Direction::Down if y < max_y - 1 => Some((y + 1, x)),
            Direction::Left if x > 0 => Some((y, x - 1)),
            Direction::Right if x < max_x - 1 => Some((y, x + 1)),
            _ => None,
        }
    }

    fn walk(&mut self, map: &Map) -> bool {
        let Some(ahead) = self.ahead(map) else {
            return false;
        };
        if map.tiles[ahead] != '.' {
            self.dir = self.dir.rotate_right_90();
        } else {
            self.pos = ahead;
        }
        true
    }
}

struct Map {
    tiles: Array2<char>,
}

impl Map {
    fn shape(&self) -> (usize, usize) {
        let shape = self.tiles.shape();
        (shape[0], shape[1])
    }
}

struct Input {
    map: Map,
    guard: Person,
}

impl From<&str> for Input {
    fn from(value: &str) -> Self {
        let chars: Vec<_> = value.lines().flat_map(|line| line.chars()).collect();
        let height = value.lines().count();
        let width = chars.len() / height;
        let mut tiles = Array2::from_shape_vec((height, width), chars).unwrap();
        let mut pos = (0, 0);
        for (coord, tile) in tiles.indexed_iter_mut() {
            if *tile == '^' {
                pos = coord;
                *tile = '.';
                break;
            }
        }

        let map = Map { tiles };
        let guard = Person {
            pos,
            dir: Direction::Up,
        };

        Self { map, guard }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let Input { map, mut guard } = input.into();
    let mut visited = HashSet::new();
    visited.insert(guard.pos);
    while guard.walk(&map) {
        visited.insert(guard.pos);
    }
    Some(visited.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let Input { mut map, mut guard } = input.into();
    let mut blacklist = HashSet::new();
    let mut visited = HashSet::new();
    let mut looped = 0;
    while let Some(ahead) = guard.ahead(&map) {
        blacklist.insert(guard.pos);
        visited.insert(guard);

        if map.tiles[ahead] == '.' && !blacklist.contains(&ahead) {
            map.tiles[ahead] = 'O';

            let mut guard2 = guard;
            let mut visited2 = visited.clone();
            'time: while guard2.walk(&map) {
                if !visited2.insert(guard2) {
                    looped += 1;
                    break 'time;
                }
            }

            map.tiles[ahead] = '.';
        }

        guard.walk(&map);
    }
    Some(looped)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
