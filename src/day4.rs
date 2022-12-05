use load_file::load_str;

#[derive(Debug, Copy, Clone)]
struct Range {
    lower: usize,
    upper: usize,
}

impl Range {
    pub fn new(txt: &str) -> Self {
        //txt eg "25-34"
        let mut lr = txt.split('-');
        let lower: usize = lr.next().unwrap().parse().unwrap();
        let upper: usize = lr.next().unwrap().parse().unwrap();
        Self { lower, upper }
    }
    pub fn contains(&self, other: Range) -> bool {
        self.lower <= other.lower && self.upper >= other.upper
    }
    pub fn partially_contained(&self, other: Range) -> bool {
        self.upper >= other.lower && self.lower <= other.upper
    }
}

struct Pair {
    left: Range,
    right: Range,
}
impl Pair {
    pub fn new(line: &str) -> Self {
        ////line eg 2-3,4-5
        let mut lr = line.split(',');
        let left = Range::new(lr.next().unwrap());
        let right = Range::new(lr.next().unwrap());
        Self { left, right }
    }
}
type Containment = fn(&Pair) -> bool;
fn full(p: &Pair) -> bool {
    p.left.contains(p.right) || p.right.contains(p.left)
}
fn partial(p: &Pair) -> bool {
    p.left.partially_contained(p.right) || p.right.partially_contained(p.left)
}

fn eval(lines: &Vec<&str>, f: Containment) {
    let mut total = 0;
    lines.iter().for_each(|line| {
        let p = Pair::new(line);

        if f(&p) {
            total += 1;
        }
    });
    println!("{}", total)
}

pub(crate) fn run() {
    let lines = load_str!("../data/day4.txt")
        .split('\n')
        .collect::<Vec<_>>();

    let contain: fn(&Pair) -> bool = full;
    println!("Part 1");
    eval(&lines, contain);

    let contain: fn(&Pair) -> bool = partial;
    println!("Part 2");
    eval(&lines, contain);
}
#[cfg(test)]
mod tests {
    use crate::day4::{full, partial, Pair, Range};

    #[test]
    fn create() {
        // 2-4,6-8
        let p1 = Range::new("2-4");
        let p2 = Range::new("6-8");
        let p3 = Range::new("623-789");
        assert_eq!(p1.lower, 2);
        assert_eq!(p1.upper, 4);
        assert_eq!(p2.lower, 6);
        assert_eq!(p2.upper, 8);
        assert_eq!(p3.lower, 623);
        assert_eq!(p3.upper, 789);
    }
    #[test]
    fn contains() {
        // 2-4,6-8
        let p1 = Range::new("2-8");
        let p2 = Range::new("3-7");
        assert_eq!(true, p1.contains(p2));
        let p1 = Range::new("4-6");
        let p2 = Range::new("6-6");
        assert_eq!(true, p1.contains(p2));
    }
    #[test]
    fn from_line() {
        let p = Pair::new("2-3,4-5");
        assert_eq!(2, p.left.lower);
        assert_eq!(3, p.left.upper);
        assert_eq!(4, p.right.lower);
        assert_eq!(5, p.right.upper);
    }
    #[test]
    fn partially_contained() {
        // In the above example, the first two pairs (2-4,6-8 and 2-3,4-5) don't overlap,
        // while the remaining four pairs
        //     (5-7,7-9, 2-8,3-7, 6-6,4-6, and 2-6,4-8) do overlap:

        let p = Pair::new("2-4,6-8");
        assert_eq!(false, partial(&p));
        let p = Pair::new("2-3,4-5");
        assert_eq!(false, partial(&p));

        let p = Pair::new("5-7,7-9");
        assert_eq!(true, partial(&p));
        let p = Pair::new("2-8,3-7");
        assert_eq!(true, partial(&p));
        let p = Pair::new("6-6,4-6");
        assert_eq!(true, partial(&p));
        let p = Pair::new("2-6,4-8");
        assert_eq!(true, partial(&p));
    }
    #[test]
    fn full_contained() {
        //For example, 2-8 fully contains 3-7, and 6-6 is fully contained by 4-6.
        // 2-8,3-7
        // 6-6,4-6
        let p = Pair::new("2-8,3-7");
        assert_eq!(true, full(&p));

        let p = Pair::new("6-6,4-6");
        assert_eq!(true, full(&p));
    }
    #[test]
    fn example_part1() {
        let data = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;
        let lines = data.split('\n').collect::<Vec<_>>();
        let mut total = 0;
        lines.iter().for_each(|line| {
            let p = Pair::new(line);
            if full(&p) {
                total += 1;
            }
        });

        assert_eq!(2, total)
    }
    #[test]
    fn example_part2() {
        let data = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;
        let lines = data.split('\n').collect::<Vec<_>>();
        let mut total = 0;
        lines.iter().for_each(|line| {
            let p = Pair::new(line);
            if partial(&p) {
                total += 1;
            }
        });

        assert_eq!(4, total)
    }
}
