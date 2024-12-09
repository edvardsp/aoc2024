use std::collections::{HashMap, HashSet};

use ndarray::Array2;

advent_of_code::solution!(8);

type Coord = (isize, isize);

struct Map {
    tiles: Array2<char>,
    antennas: HashMap<char, Vec<Coord>>,
}

impl Map {
    fn new(tiles: Array2<char>) -> Self {
        let mut antennas = HashMap::new();
        for (pos, &tile) in tiles.indexed_iter() {
            let pos = (pos.0 as isize, pos.1 as isize);
            if tile != '.' {
                antennas
                    .entry(tile)
                    .and_modify(|e: &mut Vec<_>| e.push(pos))
                    .or_insert_with(|| vec![pos]);
            }
        }
        Self { tiles, antennas }
    }

    fn valid_coord(&self, coord: &Coord) -> bool {
        if coord.0 >= 0 && coord.1 >= 0 {
            let coord = (coord.0 as usize, coord.1 as usize);
            self.tiles.get(coord).is_some()
        } else {
            false
        }
    }

    fn antinodes(&self, repeating: bool) -> usize {
        let mut total = HashSet::new();
        for coords in self.antennas.values() {
            for (pos0, pos1) in coords
                .iter()
                .flat_map(|&pos0| coords.iter().map(move |&pos1| (pos0, pos1)))
                .filter(|(pos0, pos1)| pos0 != pos1)
            {
                let (y0, x0) = pos0;
                let (y1, x1) = pos1;
                let y_diff = y1 - y0;
                let x_diff = x1 - x0;
                if repeating {
                    let mut curr_pos = pos1;
                    while self.valid_coord(&curr_pos) {
                        total.insert(curr_pos);
                        curr_pos.0 += y_diff;
                        curr_pos.1 += x_diff;
                    }
                } else {
                    let new_pos = (y1 + y_diff, x1 + x_diff);
                    if self.valid_coord(&new_pos) {
                        total.insert(new_pos);
                    }
                }
            }
        }
        total.len()
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let chars: Vec<_> = value.lines().flat_map(|line| line.chars()).collect();
        let height = value.lines().count();
        let width = chars.len() / height;
        let shape = (height, width);
        let tiles = Array2::from_shape_vec(shape, chars).unwrap();
        Self::new(tiles)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let map: Map = input.into();
    let ans = map.antinodes(false);
    Some(ans)
}

pub fn part_two(input: &str) -> Option<usize> {
    let map: Map = input.into();
    let ans = map.antinodes(true);
    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
