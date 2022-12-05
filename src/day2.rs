use crate::day2::Choice::{Paper, Rock, Scissors};
use load_file::load_str;
use std::ops::Add;
use std::str::FromStr;
use std::thread;

const ROCK_SCORE: usize = 1;
const PAPER_SCORE: usize = 2;
const SCISSORS_SCORE: usize = 3;

const WIN: usize = 6;
const DRAW: usize = 3;
const LOOSE: usize = 0;

//Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock.
#[derive(Debug)]
pub enum Choice {
    Rock,
    Scissors,
    Paper,
}
#[derive(Debug)]
enum Code {
    A,
    B,
    C,
    X,
    Y,
    Z,
}
impl FromStr for Code {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Code::A),
            "B" => Ok(Code::B),
            "C" => Ok(Code::C),
            "X" => Ok(Code::X),
            "Y" => Ok(Code::Y),
            "Z" => Ok(Code::Z),
            _ => Err(()),
        }
    }
}

fn decode(cd: &Code) -> Choice {
    match cd {
        Code::A => Rock,
        Code::B => Paper,
        Code::C => Scissors,
        Code::X => Rock,
        Code::Y => Paper,
        Code::Z => Scissors,
    }
}
fn decode_from_letter(letter: &str) -> Choice {
    let code = Code::from_str(letter).unwrap();
    decode(&code)
}
#[derive(Debug)]
struct Turn {
    pub left: String,
    pub right: String,
}
impl Turn {
    pub fn new(left: String, right: String) -> Self {
        Self { left, right }
    }
}

#[derive(Debug, Default)]
struct Pair {
    left_score: usize,
    right_score: usize,
}

impl Add for Pair {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            left_score: self.left_score + other.left_score,
            right_score: self.right_score + other.right_score,
        }
    }
}
impl Pair {
    pub fn new(left_score: usize, right_score: usize) -> Self {
        Self {
            left_score,
            right_score,
        }
    }
    pub fn flip(&self) -> Self {
        Self {
            left_score: self.right_score.to_owned(),
            right_score: self.left_score.to_owned(),
        }
    }
}

fn evaluate_turn(turn: &Turn, f: fn(&Choice, &Code) -> Choice) -> Pair {
    let left_choice = &decode_from_letter(&turn.left);
    let right_code = Code::from_str(&turn.right).unwrap();
    let new_right_choice = f(left_choice, &right_code);
    scores(left_choice, &new_right_choice)
}
fn identity_code_fn(_left_choice: &Choice, right_code: &Code) -> Choice {
    decode(right_code)
}

fn redefine_right_choice(left_choice: &Choice, right_code: &Code) -> Choice {
    // X means you need to lose.
    // Y means you need to end the round in a draw,
    // and Z means you need to win.
    match right_code {
        //Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock.
        Code::X => match left_choice {
            Rock => Scissors,
            Scissors => Paper,
            Paper => Rock,
        },
        Code::Y => match left_choice {
            Rock => Rock,
            Scissors => Scissors,
            Paper => Paper,
        },
        Code::Z => match left_choice {
            Rock => Paper,
            Scissors => Rock,
            Paper => Scissors,
        },
        _ => panic!(""),
    }
}

fn scores(left: &Choice, right: &Choice) -> Pair {
    //Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock.
    // The score for a single round is the score for the shape you selected
    // (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome of the round
    // (0 if you lost, 3 if the round was a draw, and 6 if you won).
    match (left, right) {
        (Rock, Scissors) => Pair::new(ROCK_SCORE + WIN, SCISSORS_SCORE + LOOSE),
        (Scissors, Rock) => Pair::flip(&scores(right, left)),

        (Scissors, Paper) => Pair::new(SCISSORS_SCORE + WIN, PAPER_SCORE + LOOSE),
        (Paper, Scissors) => Pair::flip(&scores(right, left)),

        (Paper, Rock) => Pair::new(PAPER_SCORE + WIN, ROCK_SCORE + LOOSE),
        (Rock, Paper) => Pair::flip(&scores(right, left)),

        (Rock, Rock) => Pair::new(ROCK_SCORE + DRAW, ROCK_SCORE + DRAW),
        (Paper, Paper) => Pair::new(PAPER_SCORE + DRAW, PAPER_SCORE + DRAW),
        (Scissors, Scissors) => Pair::new(SCISSORS_SCORE + DRAW, SCISSORS_SCORE + DRAW),
    }
}
fn line_to_turn(line: &str) -> Turn {
    //eg "A X"
    //let ch = line.chars().nth(0).unwrap();
    Turn::new(
        line.chars().next().unwrap().to_string(),
        line.chars().nth(2).unwrap().to_string(),
    )
}
fn iterate(f: fn(&Choice, &Code) -> Choice) {
    let v: Pair = load_str!("../data/day2.txt")
        .split('\n')
        .collect::<Vec<_>>()
        .iter()
        .map(|line| line_to_turn(line))
        .fold(Default::default(), |acc, t| acc + evaluate_turn(&t, f));
    println!("{:?}", v);
}
fn part2() {
    println!("Part Two score");
    iterate(redefine_right_choice);
}

fn part1() {
    println!("Part One Score");
    iterate(identity_code_fn);
}
pub(crate) fn run() {
    part1();
    part2();
}
pub(crate) fn run_threaded() {
    let mut handlers = vec![];
    let handler1 = thread::spawn(|| {
        part1();
    });
    handlers.push(handler1);
    let handler2 = thread::spawn(|| {
        part2();
    });
    handlers.push(handler2);
    for handler in handlers {
        handler.join().unwrap();
    }
    // part1();
    // part2();
}

#[cfg(test)]
mod tests {
    use crate::day2::Choice::{Paper, Rock, Scissors};
    use crate::day2::{
        evaluate_turn, identity_code_fn, scores, Pair, Turn, DRAW, ROCK_SCORE, SCISSORS_SCORE,
    };

    #[test]
    fn check_scores() {
        let ay = scores(&Rock, &Paper);
        assert_eq!(ay.right_score, 8);
        assert_eq!(ay.left_score, ROCK_SCORE);

        let bx = scores(&Paper, &Rock);
        assert_eq!(bx.right_score, ROCK_SCORE);
        assert_eq!(bx.left_score, 8);

        let cz = scores(&Scissors, &Scissors);
        assert_eq!(cz.right_score, SCISSORS_SCORE + DRAW);
        assert_eq!(cz.left_score, SCISSORS_SCORE + DRAW);
    }
    #[test]
    fn check_turns() {
        // A Y
        // B X
        // C Z
        let t1 = Turn::new("A".to_string(), "Y".to_string());
        let t2 = Turn::new("B".to_string(), "X".to_string());
        let t3 = Turn::new("C".to_string(), "Z".to_string());

        let ax = evaluate_turn(&t1, identity_code_fn);
        let bx = evaluate_turn(&t2, identity_code_fn);
        let cz = evaluate_turn(&t3, identity_code_fn);

        assert_eq!(ax.right_score, 8);
        assert_eq!(bx.right_score, 1);
        assert_eq!(cz.right_score, 6);
    }
    #[test]
    fn check_sum_turns() {
        let t1 = Turn::new("A".to_string(), "Y".to_string());
        let t2 = Turn::new("B".to_string(), "X".to_string());
        let t3 = Turn::new("C".to_string(), "Z".to_string());
        let all_turns = vec![t1, t2, t3];
        let mut scores: Pair = Default::default();

        for t in all_turns {
            let t_score = evaluate_turn(&t, identity_code_fn);
            scores = scores + t_score;
        }
        println!("Score: {:?}", scores)
    }
}
