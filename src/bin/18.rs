use rayon::prelude::*;
use std::collections::{BinaryHeap, HashMap, HashSet};

advent_of_code::solution!(18);

const DIM: usize = 70;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Coord(usize, usize);

impl Coord {
    fn neighbors(&self) -> impl Iterator<Item = Self> {
        let Coord(x, y) = *self;
        let mut list = Vec::new();
        if x > 0 {
            list.push(Self(x - 1, y));
        }
        if x < DIM {
            list.push(Self(x + 1, y));
        }
        if y > 0 {
            list.push(Self(x, y - 1));
        }
        if y < DIM {
            list.push(Self(x, y + 1));
        }
        list.into_iter()
    }
}

impl From<&str> for Coord {
    fn from(value: &str) -> Self {
        let (x_str, y_str) = value.split_once(",").unwrap();
        let x = x_str.parse().unwrap();
        let y = y_str.parse().unwrap();
        Self(x, y)
    }
}

const START: Coord = Coord(0, 0);
const STOP: Coord = Coord(DIM, DIM);

#[derive(Eq, PartialEq)]
struct State {
    cost: usize,
    pos: Coord,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(corruptions: &[Coord]) -> Option<usize> {
    let corruptions: HashSet<Coord> = HashSet::from_iter(corruptions.iter().copied());
    let mut ledger = HashMap::new();
    let mut queue = BinaryHeap::new();
    queue.push(State {
        cost: 0,
        pos: START,
    });
    while let Some(State { cost, pos }) = queue.pop() {
        if pos == STOP {
            return Some(cost);
        }

        let stored_cost = *ledger.get(&pos).unwrap_or(&usize::MAX);
        if cost > stored_cost {
            continue;
        }

        for neighbor in pos.neighbors().filter(|coord| !corruptions.contains(coord)) {
            let ncost = cost + 1;
            let stored_ncost = *ledger.get(&neighbor).unwrap_or(&usize::MAX);
            if ncost < stored_ncost {
                ledger.insert(neighbor, ncost);
                queue.push(State {
                    cost: ncost,
                    pos: neighbor,
                });
            }
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<usize> {
    let corruptions: Vec<_> = input.lines().map(Coord::from).collect();
    shortest_path(&corruptions[..1024])
}

pub fn part_two(input: &str) -> Option<String> {
    let corruptions: Vec<_> = input.lines().map(Coord::from).collect();
    (0..corruptions.len())
        .into_par_iter()
        .find_first(|&i| shortest_path(&corruptions[..=i]).is_none())
        .map(|i| corruptions[i])
        .map(|Coord(x, y)| format!("{},{}", x, y))
}
