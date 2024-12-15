use std::collections::{HashSet, VecDeque};

use ndarray::Array2;

advent_of_code::solution!(15);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Coord(isize, isize);

impl Coord {
    fn indexable(&self) -> (usize, usize) {
        assert!(self.0 >= 0);
        assert!(self.1 >= 0);
        (self.0 as usize, self.1 as usize)
    }
}

impl std::ops::AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl std::ops::Add for Coord {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Sub for Coord {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn vector(&self) -> Coord {
        match *self {
            Direction::Up => Coord(-1, 0),
            Direction::Down => Coord(1, 0),
            Direction::Left => Coord(0, -1),
            Direction::Right => Coord(0, 1),
        }
    }
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Direction::Up,
            '<' => Direction::Left,
            'v' => Direction::Down,
            '>' => Direction::Right,
            c => panic!("Unexpected Direction tile: {}", c),
        }
    }
}

struct Map {
    tiles: Array2<char>,
    robot: Coord,
}

impl Map {
    fn new(tiles: Array2<char>) -> Self {
        let robot = tiles
            .indexed_iter()
            .find_map(|(coord, ch)| if *ch == '@' { Some(coord) } else { None })
            .map(|(y, x)| Coord(y as isize, x as isize))
            .unwrap();
        Self { tiles, robot }
    }

    fn expand(&self) -> Self {
        let (height, width) = self.tiles.dim();
        let mut tiles = Array2::from_elem((height, width * 2), '.');
        for ((y, x), ch) in self.tiles.indexed_iter() {
            match ch {
                '#' => {
                    tiles[(y, 2 * x)] = '#';
                    tiles[(y, 2 * x + 1)] = '#';
                }
                'O' => {
                    tiles[(y, 2 * x)] = '[';
                    tiles[(y, 2 * x + 1)] = ']';
                }
                '@' => {
                    tiles[(y, 2 * x)] = '@';
                }
                '.' => {}
                _ => unreachable!(""),
            }
        }
        let mut robot = self.robot;
        robot.1 *= 2;
        Self { tiles, robot }
    }

    fn move_small(&mut self, dir: Direction) {
        let mv = dir.vector();
        let mut pos = self.robot;
        loop {
            match self.tiles[pos.indexable()] {
                '#' => return,
                '.' => break,
                'O' | '@' => pos += mv,
                ch => unreachable!("Unexpected tile: {}", ch),
            }
        }

        while pos != self.robot {
            let prev = pos - mv;
            self.tiles[pos.indexable()] = self.tiles[prev.indexable()];
            self.tiles[prev.indexable()] = '.';
            pos = prev;
        }
        self.robot += mv;
    }

    fn move_large(&mut self, dir: Direction) {
        let mv = dir.vector();
        let left = Direction::Left.vector();
        let right = Direction::Right.vector();

        let mut region = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(self.robot);
        while let Some(pos) = queue.pop_front() {
            match self.tiles[pos.indexable()] {
                '#' => return,
                '.' => continue,
                '@' => {
                    queue.push_back(pos + mv);
                    region.insert((pos, '@'));
                }
                ']' => {
                    if dir == Direction::Up || dir == Direction::Down {
                        queue.push_back(pos + mv);
                        queue.push_back(pos + mv + left);
                    } else {
                        queue.push_back(pos + left + left);
                    }
                    region.insert((pos + left, '['));
                    region.insert((pos, ']'));
                }
                '[' => {
                    if dir == Direction::Up || dir == Direction::Down {
                        queue.push_back(pos + mv);
                        queue.push_back(pos + mv + right);
                    } else {
                        queue.push_back(pos + right + right);
                    }
                    region.insert((pos, '['));
                    region.insert((pos + right, ']'));
                }
                ch => unreachable!("Unexpected tile: {}", ch),
            }
        }

        for (coord, _) in &region {
            self.tiles[coord.indexable()] = '.';
        }
        for (coord, ch) in region {
            self.tiles[(coord + mv).indexable()] = ch;
        }

        self.robot += mv;
    }

    fn score(&self) -> usize {
        self.tiles
            .indexed_iter()
            .filter_map(|(coord, ch)| {
                if *ch == 'O' || *ch == '[' {
                    Some(coord)
                } else {
                    None
                }
            })
            .map(|(y, x)| 100 * y + x)
            .sum()
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let chars: Vec<_> = value.lines().flat_map(|line| line.chars()).collect();
        let height = value.lines().count();
        let width = chars.len() / height;
        let shape = (height, width);
        let tiles = Array2::from_shape_vec(shape, chars).unwrap();
        Map::new(tiles)
    }
}

struct Input {
    map: Map,
    dirs: Vec<Direction>,
}

impl From<&str> for Input {
    fn from(value: &str) -> Self {
        let (map_str, dirs_str) = value.split_once("\n\n").unwrap();
        let map = map_str.into();
        let dirs = dirs_str
            .lines()
            .flat_map(|line| line.chars())
            .map(Direction::from)
            .collect();
        Self { map, dirs }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let Input { mut map, dirs } = input.into();
    for dir in dirs {
        map.move_small(dir);
    }
    Some(map.score())
}

pub fn part_two(input: &str) -> Option<usize> {
    let Input { map, dirs } = input.into();
    let mut map = map.expand();
    for dir in dirs {
        map.move_large(dir);
    }
    Some(map.score())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_one_two() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(9021));
    }
}
