use crate::{LOWER, UPPER};
use load_file::load_str;

#[derive(Debug, Clone)]
struct RuckSack {
    all_items: String,
    common: Common,
}
#[derive(Debug, Clone)]
struct Common {
    items: String,
}
#[derive(Debug, Clone)]
struct Group {
    common: Common,
}
trait Decode {
    fn decode(&self) -> usize;
}
impl Decode for RuckSack {
    fn decode(&self) -> usize {
        self.common.decode()
    }
}
impl Decode for Group {
    fn decode(&self) -> usize {
        self.common.decode()
    }
}

impl Group {
    pub fn new(rucksacks: Vec<RuckSack>) -> Self {
        let first = rucksacks.get(0).unwrap().all_items.clone();
        let second = rucksacks.get(1).unwrap().all_items.clone();
        let third = rucksacks.get(2).unwrap().all_items.clone();
        let common_1_2 = keep_common(&first, &second);
        let common = keep_common(&common_1_2, &third);
        Group {
            common: Common::new(common),
        }
    }
}

impl Common {
    pub fn new(items: String) -> Common {
        Common { items }
    }
    pub fn decode(&self) -> usize {
        decode_from_letter(self.items.chars().next().unwrap())
    }
}
fn decode_from_letter(ch: char) -> usize {
    if LOWER.contains(ch) {
        LOWER.find(ch).unwrap() + 1
    } else {
        UPPER.find(ch).unwrap() + 27
    }
}
fn keep_common(left: &str, right: &str) -> String {
    let mut common = String::new();
    for c in left.chars() {
        if right.contains(c) && !common.contains(c) {
            common.push(c);
        }
    }
    common
}
impl RuckSack {
    pub fn new(all_items: &str) -> RuckSack {
        let len = all_items.len() / 2;
        let left = all_items[0..len].to_string();
        let right = all_items[len..].to_string();
        let common = keep_common(&left, &right);
        Self {
            all_items: all_items.to_string(),
            common: Common::new(common),
        }
    }
}
fn make_rucksacks() -> Vec<RuckSack> {
    load_str!("../data/day3.txt")
        .split('\n')
        .collect::<Vec<_>>() //each line of input
        .iter()
        .map(|s| RuckSack::new(s))
        .collect::<Vec<_>>()
}
fn get_total(items: Vec<impl Decode>) -> usize {
    items
        .iter()
        .map(|s| s.decode())
        .collect::<Vec<_>>()
        .iter()
        .sum()
}
fn part2() {
    let rucksacks = make_rucksacks();
    let mut ix = 0;
    let mut groups = vec![];
    while ix < rucksacks.len() {
        let vec = vec![
            rucksacks[ix].clone(),
            rucksacks[ix + 1].clone(),
            rucksacks[ix + 2].clone(),
        ];
        let g = Group::new(vec);
        groups.push(g);
        ix += 3;
    }
    let total = get_total(groups);

    println!("Part 2 {:?}", total);
}

fn part1() {
    let sacks = make_rucksacks();
    let total = get_total(sacks);

    println!("Part 1 {}", total);
}
pub(crate) fn run() {
    part1();
    part2();
}

#[cfg(test)]
mod tests {
    use crate::day3::{make_rucksacks, RuckSack};

    #[test]
    fn create() {
        let r1 = RuckSack::new("vJrwpWtwJgWrhcsFMMfFFhFp");
        assert_eq!("p", r1.common.items);

        let r1 = RuckSack::new("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn");
        assert_eq!("v", r1.common.items);

        let r1 = RuckSack::new("CrZsJsPPZsGzwwsLwLmpwMDw");
        assert_eq!("s", r1.common.items);
    }
    #[test]
    fn part1_check() {
        let total: usize = make_rucksacks()
            .iter()
            .map(|s| s.common.decode())
            .collect::<Vec<_>>()
            .iter()
            .sum();

        println!("{}", total);
    }
    #[test]
    fn sample_part1() {
        // vJrwpWtwJgWrhcsFMMfFFhFp

        //
        let mut all_sacks = vec![];
        let r = RuckSack::new("vJrwpWtwJgWrhcsFMMfFFhFp");
        all_sacks.push(r);
        let r = RuckSack::new("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL");
        all_sacks.push(r);
        let r = RuckSack::new("PmmdzqPrVvPwwTWBwg");
        all_sacks.push(r);
        let r = RuckSack::new("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn");
        all_sacks.push(r);
        let r = RuckSack::new("ttgJtRGJQctTZtZT");
        all_sacks.push(r);
        let r = RuckSack::new("CrZsJsPPZsGzwwsLwLmpwMDw");
        all_sacks.push(r);

        let mut total: usize = 0;
        for r in all_sacks {
            total += r.common.decode()
        }
        println!("{}", total);
    }
}
