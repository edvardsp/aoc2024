use ndarray::{array, s, Array2};

advent_of_code::solution!(4);

struct Map {
    map: Array2<char>,
}

impl Map {
    fn total_xmas(&self) -> usize {
        let xmas = array!['X', 'M', 'A', 'S'];
        let samx = array!['S', 'A', 'M', 'X'];

        let mut summa = 0;
        for row in self.map.rows() {
            for window in row.windows(4) {
                if window == xmas || window == samx {
                    summa += 1;
                }
            }
        }
        for col in self.map.columns() {
            for window in col.windows(4) {
                if window == xmas || window == samx {
                    summa += 1;
                }
            }
        }
        for window in self.map.windows((4, 4)) {
            let anti_window = window.slice(s![..;-1, ..]);
            let main_diag = window.diag();
            let anti_diag = anti_window.diag();
            if main_diag == xmas || main_diag == samx {
                summa += 1;
            }
            if anti_diag == xmas || anti_diag == samx {
                summa += 1;
            }
        }

        summa
    }

    fn total_max_cross(&self) -> usize {
        let mas = array!['M', 'A', 'S'];
        let sam = array!['S', 'A', 'M'];

        let mut summa = 0;
        for window in self.map.windows((3, 3)) {
            let anti_window = window.slice(s![..;-1, ..]);
            let main_diag = window.diag();
            let anti_diag = anti_window.diag();
            if (main_diag == mas || main_diag == sam) && (anti_diag == mas || anti_diag == sam) {
                summa += 1;
            }
        }
        summa
    }
}

impl From<&str> for Map {
    fn from(s: &str) -> Self {
        let lines: Vec<_> = s.lines().collect();
        let shape = (lines.len(), lines[0].len());
        let vec = lines.into_iter().flat_map(|l| l.chars()).collect();
        let map = Array2::from_shape_vec(shape, vec).unwrap();
        Self { map }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let map: Map = input.into();
    Some(map.total_xmas())
}

pub fn part_two(input: &str) -> Option<usize> {
    let map: Map = input.into();
    Some(map.total_max_cross())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
