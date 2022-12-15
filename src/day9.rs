use crate::day9::Cmd::{D, L, R, U};

use load_file::load_str;
use regex::Regex;
use std::collections::HashSet;
use std::hash::Hash;
use std::ops::Sub;

#[derive(Debug, Clone)]

struct Snake {
    tail: Vec<Point>,
    visited: HashSet<Point>,
}
impl Snake {
    pub fn new(tail_len: usize) -> Self {
        let mut tail = Vec::new();
        for _ in 0..tail_len + 1 {
            tail.push(Point::new());
        }

        Self {
            tail,
            visited: Default::default(),
        }
    }
    pub fn accept_cmd(&mut self, c: &Cmd) {
        match c {
            U(1) => {
                self.tail[0].y -= 1;
            }
            D(1) => {
                self.tail[0].y += 1;
            }
            L(1) => {
                self.tail[0].x -= 1;
            }
            R(1) => {
                self.tail[0].x += 1;
            }

            _ => unreachable!(),
        }
    }
    pub fn tail_follow(&mut self) {
        for i in 1..self.tail.len() {
            let (dx, dy) = self.get_dxdy(self.tail[i - 1], self.tail[i]);

            self.tail[i].x += dx;
            self.tail[i].y += dy;
            if i == self.tail.len() - 1 {
                self.visited.insert(self.tail[i]);
            }
        }
    }
    fn get_dxdy(&self, p1: Point, p2: Point) -> (i16, i16) {
        let diff = p1 - p2;
        match (diff.x, diff.y) {
            (0, 0) => (0, 0),
            (0, 1) | (1, 0) | (0, -1) | (-1, 0) => (0, 0),
            (1, 1) | (1, -1) | (-1, 1) | (-1, -1) => (0, 0),
            (0, 2) => (0, 1),
            (0, -2) => (0, -1),
            (2, 0) => (1, 0),
            (-2, 0) => (-1, 0),
            (2, 1) => (1, 1),
            (2, -1) => (1, -1),
            (-2, 1) => (-1, 1),
            (-2, -1) => (-1, -1),
            (1, 2) => (1, 1),
            (-1, 2) => (-1, 1),
            (1, -2) => (1, -1),
            (-1, -2) => (-1, -1),
            (-2, -2) => (-1, -1),
            (-2, 2) => (-1, 1),
            (2, -2) => (1, -1),
            (2, 2) => (1, 1),
            _ => panic!("unhandled case: tail - head = {diff:?}"),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    pub x: i16,
    pub y: i16,
}
impl Point {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }
}
impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        //+1 if -ve -1 if +ve
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Cmd {
    U(i16),
    D(i16),
    L(i16),
    R(i16),
}
impl From<&str> for Cmd {
    fn from(s: &str) -> Cmd {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^([U|D|L|R]) (\d+)$").unwrap();
        }
        let caps = RE.captures(s).unwrap();
        let c = caps.get(1).unwrap().as_str();
        let d = caps.get(2).unwrap().as_str().parse::<i16>().unwrap();
        match c {
            "U" => U(d),
            "D" => D(d),
            "L" => L(d),
            "R" => R(d),
            _ => unreachable!(),
        }
    }
}
fn parse(l: &str) -> Cmd {
    Cmd::from(l)
}
fn extend(c: &Cmd) -> Vec<Cmd> {
    let mut res = vec![];
    match c {
        U(n) => {
            for i in 0..*n {
                res.push(U(1));
            }
        }
        D(n) => {
            for i in 0..*n {
                res.push(D(1));
            }
        }
        L(n) => {
            for i in 0..*n {
                res.push(L(1));
            }
        }
        R(n) => {
            for i in 0..*n {
                res.push(R(1));
            }
        }
    };
    res
}
fn eval_with_tail_len(cmds: &Vec<Cmd>, tail_size: usize) -> usize {
    let mut snake = Snake::new(tail_size);

    for cmd in cmds {
        snake.accept_cmd(cmd);
        snake.tail_follow();
    }

    snake.visited.len()
}

pub fn run() {
    let cmds = load_str!("../data/day9.txt")
        .split('\n')
        .collect::<Vec<_>>()
        .iter()
        .map(|l| parse(l))
        .collect::<Vec<Cmd>>()
        .iter()
        .flat_map(extend)
        .collect::<Vec<Cmd>>();
    println!("Part 1 {}", eval_with_tail_len(&cmds, 1));
    println!("Part 2 {}", eval_with_tail_len(&cmds, 9));
}
#[cfg(test)]
mod tests {
    use crate::day9::parse;
    use crate::day9::Cmd::*;

    #[test]
    fn create() {
        let c = parse("U 3");
        assert_eq!(c, U(3));
        let c = parse("D 1");
        assert_eq!(c, D(1));
        let c = parse("L 322");
        assert_eq!(c, L(322));
        let c = parse("R 4");
        assert_eq!(c, R(4));
    }
}
