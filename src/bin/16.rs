use std::collections::{BinaryHeap, HashMap, HashSet};

use ndarray::Array2;

advent_of_code::solution!(16);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    fn clockwise(&self) -> Self {
        match *self {
            Dir::North => Dir::East,
            Dir::East => Dir::South,
            Dir::South => Dir::West,
            Dir::West => Dir::North,
        }
    }

    fn counterclockwise(&self) -> Self {
        match *self {
            Dir::North => Dir::West,
            Dir::West => Dir::South,
            Dir::South => Dir::East,
            Dir::East => Dir::North,
        }
    }

    fn walk(&self, coord: (usize, usize)) -> (usize, usize) {
        let (y, x) = coord;
        match *self {
            Dir::North => (y - 1, x),
            Dir::South => (y + 1, x),
            Dir::East => (y, x + 1),
            Dir::West => (y, x - 1),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    pos: (usize, usize),
    dir: Dir,
    path: Vec<(usize, usize)>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| other.path.len().cmp(&self.path.len()))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct Map {
    tiles: Array2<char>,
}

impl Map {
    fn score(&self) -> Option<usize> {
        let (height, width) = self.tiles.dim();
        let start = (height - 2, 1);
        let stop = (1, width - 2);

        let mut dist = HashMap::new();
        let mut queue = BinaryHeap::new();
        queue.push(State {
            cost: 0,
            pos: start,
            dir: Dir::East,
            path: vec![],
        });

        while let Some(State {
            cost,
            pos,
            dir,
            path: _,
        }) = queue.pop()
        {
            if pos == stop {
                return Some(cost);
            }

            if cost > *dist.get(&(pos, dir)).unwrap_or(&usize::MAX) {
                continue;
            }

            let ahead = dir.walk(pos);
            let candidates = [
                State {
                    cost: cost + 1,
                    pos: ahead,
                    dir,
                    path: vec![],
                },
                State {
                    cost: cost + 1000,
                    pos,
                    dir: dir.clockwise(),
                    path: vec![],
                },
                State {
                    cost: cost + 1000,
                    pos,
                    dir: dir.counterclockwise(),
                    path: vec![],
                },
            ];
            for next in candidates {
                if self.tiles[next.pos] == '#' {
                    continue;
                }
                if next.cost < *dist.get(&(next.pos, next.dir)).unwrap_or(&usize::MAX) {
                    dist.insert((next.pos, next.dir), next.cost);
                    queue.push(next);
                }
            }
        }

        None
    }

    fn best_paths(&self) -> usize {
        let (height, width) = self.tiles.dim();
        let start = (height - 2, 1);
        let stop = (1, width - 2);

        let mut lowest_score = None;
        let mut tiles_on_path = HashSet::new();
        let mut ledger = HashMap::new();

        let mut queue = BinaryHeap::new();
        queue.push(State {
            cost: 0,
            pos: start,
            dir: Dir::East,
            path: vec![start],
        });
        while let Some(State {
            cost,
            pos,
            dir,
            path,
        }) = queue.pop()
        {
            if pos == stop {
                if cost == lowest_score.unwrap_or(cost) {
                    lowest_score = Some(cost);
                    tiles_on_path.extend(path.iter().copied());
                }
                continue;
            }

            let stored_cost = *ledger.get(&(pos, dir)).unwrap_or(&usize::MAX);
            if cost > stored_cost {
                continue;
            }

            for (rotation, rotation_cost) in [
                (dir, 0),
                (dir.clockwise(), 1000),
                (dir.counterclockwise(), 1000),
                (dir.clockwise().clockwise(), 2000),
            ] {
                let stored_cost = *ledger.get(&(pos, rotation)).unwrap_or(&usize::MAX);
                let new_cost = cost + rotation_cost;
                if new_cost < stored_cost {
                    ledger.insert((pos, rotation), new_cost);
                }

                let ahead = rotation.walk(pos);
                if self.tiles[ahead] != '#' {
                    let stored_cost = *ledger.get(&(ahead, rotation)).unwrap_or(&usize::MAX);
                    let new_cost = cost + rotation_cost + 1;
                    if new_cost <= stored_cost {
                        let mut new_path = path.clone();
                        new_path.push(ahead);
                        queue.push(State {
                            cost: new_cost,
                            pos: ahead,
                            dir: rotation,
                            path: new_path,
                        });
                        ledger.insert((ahead, rotation), new_cost);
                    }
                }
            }
        }

        tiles_on_path.len()
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let chars: Vec<_> = value.lines().flat_map(|line| line.chars()).collect();
        let height = value.lines().count();
        let width = chars.len() / height;
        let shape = (height, width);
        let tiles = Array2::from_shape_vec(shape, chars).unwrap();
        Self { tiles }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let map: Map = input.into();
    Some(map.score().unwrap())
}

pub fn part_two(input: &str) -> Option<usize> {
    let map: Map = input.into();
    Some(map.best_paths())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_one_two() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two_one() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(45));
    }

    #[test]
    fn test_part_two_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(64));
    }
}
