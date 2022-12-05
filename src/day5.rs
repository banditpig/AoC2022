use lazy_static::lazy_static;
use load_file::load_str;
use std::fmt::Debug;

use crate::day5::Crane::{CrateMover9000, CrateMover9001};
use regex::Regex;
#[derive(PartialEq)]
enum Crane {
    //CrateMover 9000 - it's a CrateMover 9001.
    CrateMover9000,
    CrateMover9001,
}
#[derive(Debug)]
struct Action {
    quantity: usize,
    from: usize,
    to: usize,
}

impl Action {
    pub fn new(quantity: usize, from: usize, to: usize) -> Action {
        Self { quantity, from, to }
    }
}
fn create_actions() -> Vec<Action> {
    load_str!("../data/day5.txt")
        .split('\n')
        .collect::<Vec<_>>()
        .iter()
        .map(|l| parse_line(l))
        .collect::<Vec<Action>>() //each line of input
}
fn parse_line(line: &str) -> Action {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    }
    let caps = RE.captures(line).unwrap();
    //the whole thing is in caps.get(0)
    let quantity = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
    let from = caps.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
    let to = caps.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1;
    Action::new(quantity, from, to)
}
fn eval(crane: Crane, cargo: &mut Cargo<char>, actions: &Vec<Action>) {
    // let mut cargo = setup_cargo();
    // let actions = create_actions();
    for action in actions {
        cargo.accept_action(action, &crane);
    }
    let m = cargo.get_message().into_iter().collect::<String>();
    println!("{}", m);
}
pub(crate) fn run() {
    let mut cargo = setup_cargo();
    let actions = create_actions();
    println!("Part 1");
    eval(CrateMover9000, &mut cargo, &actions);
    println!();
    let mut cargo = setup_cargo();
    println!("Part 2");
    eval(CrateMover9001, &mut cargo, &actions);
}
#[derive(Debug)]
struct Stack<T> {
    id: usize,
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new(id: usize) -> Stack<T> {
        Self {
            id: id,
            items: Vec::new(),
        }
    }
    pub fn push(&mut self, x: T) {
        self.items.push(x);
    }
    //Dont expect the puzzle input to cause error here so just unwrap.
    pub fn pop(&mut self) -> T {
        self.items.pop().unwrap()
    }
    pub fn flip(&mut self) {
        self.items.reverse()
    }
}
#[derive(Debug)]
struct Cargo<T: Debug> {
    stacks: Vec<Stack<T>>,
}

impl<T: Debug> Cargo<T> {
    pub fn new(count: usize) -> Cargo<T> {
        let mut stacks: Vec<Stack<T>> = Vec::new();
        for i in 0..count {
            stacks.push(Stack::new(i));
        }
        Self { stacks }
    }
    pub fn accept_action(&mut self, act: &Action, crane: &Crane) {
        self.move_items(act.quantity, act.from, act.to, crane);
    }
    pub fn move_items(&mut self, count: usize, from: usize, to: usize, crane: &Crane) {
        let mut items = self.pop(from, count);
        if *crane == Crane::CrateMover9001 {
            items.reverse();
        }
        self.push(to, items);
    }

    pub fn get_message(&mut self) -> Vec<T> {
        let mut message = Vec::new();
        for s in self.stacks.iter_mut() {
            message.push(s.pop());
        }
        message
    }
    fn push(&mut self, stack_id: usize, items: Vec<T>) {
        let mut stack = self.stacks.get_mut(stack_id).unwrap();
        for item in items {
            stack.push(item);
        }
    }

    fn pop(&mut self, stack_id: usize, count: usize) -> Vec<T> {
        let mut values = Vec::new();
        let stack = self.stacks.get_mut(stack_id).unwrap();
        for i in 0..count {
            values.push(stack.pop())
        }
        values
    }
}
fn setup_cargo() -> Cargo<char> {
    let s1 = ['S', 'M', 'R', 'N', 'W', 'J', 'V', 'T'].to_vec();
    let s2 = ['B', 'W', 'D', 'J', 'Q', 'P', 'C', 'V'].to_vec();
    let s3 = ['B', 'J', 'F', 'H', 'D', 'R', 'P'].to_vec();
    let s4 = ['F', 'R', 'P', 'B', 'M', 'N', 'D'].to_vec();
    let s5 = ['H', 'V', 'R', 'P', 'T', 'B'].to_vec();
    let s6 = ['C', 'B', 'P', 'T'].to_vec();
    let s7 = ['B', 'J', 'R', 'P', 'L'].to_vec();
    let s8 = ['N', 'C', 'S', 'L', 'T', 'Z', 'B', 'W'].to_vec();
    let s9 = ['L', 'S', 'G'].to_vec();
    //let data = vec![s1, s2, s3, s4, s5, s6,s7, s8, s9];

    let mut cargo: Cargo<char> = Cargo::new(9);
    cargo.push(0, s1);
    cargo.push(1, s2);
    cargo.push(2, s3);
    cargo.push(3, s4);
    cargo.push(4, s5);
    cargo.push(5, s6);
    cargo.push(6, s7);
    cargo.push(7, s8);
    cargo.push(8, s9);
    cargo
}
#[cfg(test)]
mod tests {
    use crate::day5::{parse_line, setup_cargo, Cargo, Stack};
    use load_file::load_str;
    use std::env::VarError;

    #[test]
    fn cargo() {
        let c = setup_cargo();
        println!("{:?}", c);

        println!("{:?}", 1)
    }
    #[test]
    fn parse() {
        let a = parse_line("move 1 from 2 to 1");
        assert_eq!(a.quantity, 1);
        assert_eq!(a.from, 2);
        assert_eq!(a.to, 1);

        let a = parse_line("move 6 from 3 to 5");
        assert_eq!(a.quantity, 6);
        assert_eq!(a.from, 3);
        assert_eq!(a.to, 5);

        let a = parse_line("move 3 from 3 to 12");
        assert_eq!(a.quantity, 3);
        assert_eq!(a.from, 3);
        assert_eq!(a.to, 12);
    }
    #[test]
    fn read_input() {
        let lines = load_str!("../data/day5.txt")
            .split('\n')
            .collect::<Vec<_>>();

        println!("{:?}", lines);
    }
    #[test]
    fn stack() {
        let mut s: Stack<char> = Stack::new(0);
        s.push('a');
        s.push('b');
        s.push('c');

        assert_eq!(s.pop(), 'c');
        assert_eq!(s.pop(), 'b');
        assert_eq!(s.pop(), 'a');

        s.push('a');
        s.push('b');
        s.push('c');
        s.flip();
        assert_eq!(s.pop(), 'a');
        assert_eq!(s.pop(), 'b');
        assert_eq!(s.pop(), 'c');
    }
    // [T] [V]                     [W]
    // [V] [C] [P] [D]             [B]
    // [J] [P] [R] [N] [B]         [Z]
    // [W] [Q] [D] [M] [T]     [L] [T]
    // [N] [J] [H] [B] [P] [T] [P] [L]
    // [R] [D] [F] [P] [R] [P] [R] [S] [G]
    // [M] [W] [J] [R] [V] [B] [J] [C] [S]
    // [S] [B] [B] [F] [H] [C] [B] [N] [L]
    //  1   2   3   4   5   6   7   8   9
    #[test]
    fn show_cargo() {
        //     [D]
        // [N] [C]
        // [Z] [M] [P]
        // 1   2   3
        let mut c: Cargo<char> = Cargo::new(3);
        //let a1 =  S M R N W J V T
        c.push(0, ['Z', 'N'].to_vec());
        c.push(1, ['M', 'C', 'D'].to_vec());
        c.push(2, ['P'].to_vec());

        //move 1 from 2 to 1
        c.move_items(1, 2 - 1, 1 - 1);
        // move 3 from 1 to 3
        c.move_items(3, 1 - 1, 3 - 1);
        // move 2 from 2 to 1
        c.move_items(2, 2 - 1, 1 - 1);
        // move 1 from 1 to 2
        c.move_items(1, 1 - 1, 2 - 1);
        println!("{:?}", c);
        println!("{:?}", c.get_message().into_iter().collect::<String>());
    }
}
