// A tree is visible if all of the other trees between
// it and an edge of the grid are shorter than it.
// Only consider trees in the same row or column;
// that is, only look up, down, left, or right from any given tree.

use crate::day8::Direction::{DOWN, LEFT, RIGHT, UP};
use itertools::chain;

#[derive(Debug, Clone)]
struct Map {
    row_count: i16,
    col_count: i16,
    trees: Vec<Vec<u32>>,
}
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
impl Map {
    pub fn new(input: &str) -> Map {
        let lines = input.split('\n').collect::<Vec<_>>();
        let mut trees: Vec<Vec<u32>> = vec![];
        lines.iter().for_each(|line| {
            let r = line
                .chars()
                .into_iter()
                .collect::<Vec<char>>()
                .iter()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>();
            trees.push(r);
        });
        //assume square.
        Self {
            row_count: trees.len() as i16,
            col_count: trees.len() as i16,
            trees,
        }
    }

    fn height(&self, row: i16, col: i16) -> u32 {
        *self
            .trees
            .get(row as usize)
            .unwrap()
            .get(col as usize)
            .unwrap()
    }
    fn visible_counts(&self, d: Direction, r: i16, c: i16) -> i32 {
        let mut ri = r;
        let mut ci = c;
        let this_height = self.height(r, c);
        let mut cnt = 0;
        match d {
            UP => {
                ri -= 1;
                while ri >= 0 {
                    if self.height(ri, ci) >= this_height {
                        return cnt;
                    }
                    cnt += 1;
                    ri -= 1;
                }
                return cnt;
            } //- r fix c
            DOWN => {
                ri += 1;
                while ri < self.row_count {
                    if self.height(ri, ci) >= this_height {
                        return cnt;
                    }
                    cnt += 1;
                    ri += 1;
                }
                return cnt;
            } //+ r fix c
            LEFT => {
                ci -= 1;
                while ci >= 0 {
                    if self.height(ri, ci) >= this_height {
                        return cnt;
                    }
                    cnt += 1;
                    ci -= 1;
                }
                return cnt;
            } //fix r - c
            RIGHT => {
                ci += 1;
                while ci < self.col_count {
                    let h = self.height(ri, ci);
                    if self.height(ri, ci) >= this_height {
                        return cnt;
                    }
                    cnt += 1;
                    ci += 1;
                }
                return cnt;
            } //fix r + c
        };
        cnt
    }
    fn visible(&self, d: Direction, r: i16, c: i16) -> bool {
        let mut ri = r;
        let mut ci = c;
        let this_height = self.height(r, c);
        match d {
            UP => {
                ri -= 1;
                while ri >= 0 {
                    if self.height(ri, ci) >= this_height {
                        return false;
                    }
                    ri -= 1;
                }
            } //- r fix c
            DOWN => {
                ri += 1;
                while ri < self.row_count {
                    if self.height(ri, ci) >= this_height {
                        return false;
                    }
                    ri += 1;
                }
            } //+ r fix c
            LEFT => {
                ci -= 1;
                while ci >= 0 {
                    if self.height(ri, ci) >= this_height {
                        return false;
                    }
                    ci -= 1;
                }
            } //fix r - c
            RIGHT => {
                ci += 1;
                while ci < self.col_count {
                    if self.height(ri, ci) >= this_height {
                        return false;
                    }
                    ci += 1;
                }
            } //fix r + c
        }
        true
    }
    fn on_edge(&self, r: i16, c: i16) -> bool {
        r == 0 || r == self.row_count - 1 || c == 0 || c == self.col_count - 1
    }
    fn get_scenic_distances(&self) -> Vec<i32> {
        let mut total = 0;
        let mut totals = vec![];
        for r in 0..self.row_count {
            for c in 0..self.col_count {
                let scenic = self.visible_counts(UP, r, c)
                    * self.visible_counts(DOWN, r, c)
                    * self.visible_counts(LEFT, r, c)
                    * self.visible_counts(RIGHT, r, c);
                totals.push(scenic);
            }
        }
        totals.sort();
        println!("{:?}", totals);
        totals
    }
    fn count_all_visible_trees(&self) -> usize {
        let mut total = 0;
        for r in 0..self.row_count {
            for c in 0..self.col_count {
                if self.visible(UP, r, c)
                    || self.visible(DOWN, r, c)
                    || self.visible(LEFT, r, c)
                    || self.visible(RIGHT, r, c)
                {
                    total += 1;
                }
            }
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use crate::day8::{Direction, Map};
    use load_file::load_str;

    #[test]
    fn check_loads() {
        let s = load_str!("../data/day8.txt");
        let m = Map::new(s);
        println!("{:?}", m);
    }

    #[test]
    fn scenics() {
        let s = load_str!("../data/day8.txt");
        let m = Map::new(s);
        let t = m.get_scenic_distances();
        println!("{:?}", t);
        //
    }
    #[test]
    fn all_visible() {
        let s = load_str!("../data/day8.txt");
        let m = Map::new(s);
        let t = m.count_all_visible_trees();
        println!("{t}");
    }
    #[test]
    fn visible_counts() {
        //visible_counts
        let s = load_str!("../data/day8.txt");
        let m = Map::new(s);
        let u = m.visible_counts(Direction::UP, 3, 2);
        assert_eq!(u, 2);

        let l = m.visible_counts(Direction::LEFT, 3, 2);
        assert_eq!(l, 2);

        let r = m.visible_counts(Direction::RIGHT, 3, 2);
        assert_eq!(r, 2);

        let d = m.visible_counts(Direction::DOWN, 3, 2);
        assert_eq!(d, 1);

        println!("{}", u * l * r * d);
    }
    #[test]
    fn visible() {
        let s = load_str!("../data/day8.txt");
        let m = Map::new(s);
        assert_eq!(true, m.visible(Direction::UP, 1, 1));
        assert_eq!(true, m.visible(Direction::LEFT, 1, 1));
        assert_eq!(true, m.visible(Direction::UP, 1, 2));
        assert_eq!(true, m.visible(Direction::RIGHT, 1, 2));

        assert_eq!(false, m.visible(Direction::RIGHT, 1, 3));
        assert_eq!(false, m.visible(Direction::UP, 1, 3));
        assert_eq!(false, m.visible(Direction::LEFT, 1, 3));
        assert_eq!(false, m.visible(Direction::DOWN, 1, 3));

        assert_eq!(true, m.visible(Direction::RIGHT, 2, 1));
        assert_eq!(false, m.visible(Direction::LEFT, 2, 1));
        assert_eq!(false, m.visible(Direction::UP, 2, 1));
        assert_eq!(false, m.visible(Direction::DOWN, 2, 1));

        assert_eq!(true, m.visible(Direction::RIGHT, 2, 3));
        //
        // assert_eq!(true, m.visible(Direction::RIGHT, 1, 2));
    }
    // 30373
    // 25512
    // 65332
    // 33549
    // 35390
    #[test]
    fn edge() {
        let s = load_str!("../data/day8.txt");
        let m = Map::new(s);
        assert_eq!(true, m.on_edge(0, 0));
        assert_eq!(true, m.on_edge(0, 2));
        assert_eq!(true, m.on_edge(0, 3));
        assert_eq!(true, m.on_edge(0, 4));

        assert_eq!(true, m.on_edge(2, 0));
        assert_eq!(true, m.on_edge(2, 0));
        assert_eq!(true, m.on_edge(4, 1));

        assert_eq!(false, m.on_edge(3, 3));
        assert_eq!(false, m.on_edge(3, 2));
        assert_eq!(false, m.on_edge(2, 2));
    }
}
