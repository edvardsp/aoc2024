use std::collections::{HashSet, VecDeque};

use ndarray::{s, Array2};

advent_of_code::solution!(12);

type Coord = (usize, usize);

fn neighbors(coord: &Coord) -> impl Iterator<Item = Coord> {
    let (y, x) = *coord;
    [(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)].into_iter()
}

struct Region {
    coords: HashSet<Coord>,
}

impl Region {
    fn bounding_box(&self) -> (Coord, Coord) {
        let (ymin, xmin) = self
            .coords
            .iter()
            .fold((usize::MAX, usize::MAX), |(ymin, xmin), (y, x)| {
                (ymin.min(*y), xmin.min(*x))
            });
        let (ymax, xmax) = self
            .coords
            .iter()
            .fold((usize::MIN, usize::MIN), |(ymax, xmax), (y, x)| {
                (ymax.max(*y), xmax.max(*x))
            });

        let upper_left = (ymin, xmin);
        let lower_right = (ymax, xmax);
        (upper_left, lower_right)
    }

    fn perimeter(&self) -> usize {
        self.coords
            .iter()
            .map(|coord| {
                neighbors(coord)
                    .filter(|coord| !self.coords.contains(coord))
                    .count()
            })
            .sum()
    }

    fn sides(&self) -> usize {
        let ((ymin, xmin), (ymax, xmax)) = self.bounding_box();

        let mut total = 0;
        for y in ymin..=ymax {
            for x in xmin..=xmax {
                let coord = (y, x);
                let curr = self.coords.contains(&coord);
                let below = self.coords.contains(&(y + 1, x));
                let above = self.coords.contains(&(y - 1, x));
                let front = self.coords.contains(&(y, x + 1));
                let behind = self.coords.contains(&(y, x - 1));
                if curr {
                    if !behind && !above {
                        total += 2;
                    }
                    if !front && !below {
                        total += 2;
                    }
                } else {
                    let lower_left = self.coords.contains(&(y + 1, x - 1));
                    let upper_right = self.coords.contains(&(y - 1, x + 1));
                    if behind && below && lower_left {
                        total += 2;
                    }
                    if above && front && upper_right {
                        total += 2;
                    }
                }
            }
        }
        total
    }

    fn area(&self) -> usize {
        self.coords.len()
    }

    fn price(&self) -> usize {
        self.area() * self.perimeter()
    }

    fn bulk_discount(&self) -> usize {
        self.area() * self.sides()
    }
}

struct Map(Array2<char>);

impl Map {
    fn regions(&self) -> impl Iterator<Item = Region> {
        let mut regions = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back((1, 1));

        while let Some(origin) = queue.pop_front() {
            if visited.contains(&origin) {
                continue;
            }

            let mark = self.0[origin];
            let mut coords = HashSet::new();
            let mut flood = VecDeque::new();
            flood.push_back(origin);
            while let Some(search) = flood.pop_front() {
                let tile = self.0[search];
                if tile == '.' {
                    continue;
                }

                if tile != mark {
                    if !visited.contains(&search) {
                        queue.push_back(search);
                    }
                    continue;
                }

                if !visited.insert(search) {
                    continue;
                }
                coords.insert(search);
                flood.extend(neighbors(&search).filter(|c| !visited.contains(c)));
            }

            regions.push(Region { coords });
        }

        regions.into_iter()
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let chars: Vec<_> = value.lines().flat_map(|line| line.chars()).collect();
        let height = value.lines().count();
        let width = chars.len() / height;
        let shape = (height, width);
        let map = Array2::from_shape_vec(shape, chars).unwrap();
        let (rows, cols) = map.dim();
        let mut padded = Array2::from_elem((rows + 2, cols + 2), '.');
        padded.slice_mut(s![1..-1, 1..-1]).assign(&map);
        Self(padded)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let map: Map = input.into();
    let ans = map.regions().map(|region| region.price()).sum();
    Some(ans)
}

pub fn part_two(input: &str) -> Option<usize> {
    let map: Map = input.into();
    let ans = map.regions().map(|region| region.bulk_discount()).sum();
    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
