use ndarray::Array2;
use std::{cmp::Ordering, collections::HashSet};

advent_of_code::solution!(14);

const WIDTH: isize = 101;
const HEIGHT: isize = 103;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Vec2 {
    x: isize,
    y: isize,
}

impl std::ops::AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl From<&str> for Vec2 {
    fn from(value: &str) -> Self {
        let (x, y) = value.split_once(",").unwrap();
        let x = x.parse().unwrap();
        let y = y.parse().unwrap();
        Self { x, y }
    }
}

#[derive(Clone, Debug)]
struct Robot {
    pos: Vec2,
    vel: Vec2,
}

impl Robot {
    fn tick(&mut self) {
        self.pos += self.vel;
        self.pos.x = self.pos.x.rem_euclid(WIDTH);
        self.pos.y = self.pos.y.rem_euclid(HEIGHT);
    }

    fn quadrant(&self) -> Option<usize> {
        let horizontal = self.pos.x.cmp(&(WIDTH / 2));
        let vertical = self.pos.y.cmp(&(HEIGHT / 2));
        match (horizontal, vertical) {
            (Ordering::Equal, _) | (_, Ordering::Equal) => None,
            (Ordering::Less, Ordering::Less) => Some(0),
            (Ordering::Greater, Ordering::Less) => Some(1),
            (Ordering::Less, Ordering::Greater) => Some(2),
            (Ordering::Greater, Ordering::Greater) => Some(3),
        }
    }
}

impl From<&str> for Robot {
    fn from(value: &str) -> Self {
        let (pos, vel) = value.split_once(" ").unwrap();
        let pos = pos.strip_prefix("p=").unwrap().into();
        let vel = vel.strip_prefix("v=").unwrap().into();
        Self { pos, vel }
    }
}

#[allow(dead_code)]
fn print_map(map: &Array2<char>) {
    let mut str = String::new();
    for row in map.rows() {
        for c in row {
            str.push(*c);
        }
        str.push('\n');
    }
    println!("{}", str);
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut quadrants = [0; 4];
    for mut robot in input.lines().map(Robot::from) {
        for _ in 0..100 {
            robot.tick();
        }
        if let Some(quad) = robot.quadrant() {
            quadrants[quad] += 1;
        }
    }
    Some(quadrants.into_iter().product())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut robots: Vec<_> = input.lines().map(Robot::from).collect();

    for i in 1.. {
        for robot in &mut robots {
            robot.tick();
        }

        let mut visited = HashSet::new();
        if robots.iter().all(|robot| visited.insert(robot.pos)) {
            // let mut map = Array2::from_elem((HEIGHT as usize, WIDTH as usize), ' ');
            // for robot in &robots {
            //     let pos = (robot.pos.y as usize, robot.pos.x as usize);
            //     map[pos] = 'X';
            // }
            // print_map(&map);
            return Some(i);
        }
    }
    None
}
