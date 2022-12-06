use itertools::Itertools;
use load_file::load_str;

struct Stream {
    data: Vec<char>,
}

impl Stream {
    pub fn new(s: &str) -> Stream {
        let data = s.chars().collect::<Vec<char>>();
        Self { data }
    }
    fn all_different(&self, s: &[char], window: usize) -> bool {
        //this will remove duplicates
        let v = s.to_vec().into_iter().unique().collect::<Vec<char>>();
        //nothing removed?
        v.len() == window
    }

    pub fn process(&mut self, window: usize) -> usize {
        let mut found = false;
        let mut ix = 0;
        while !found {
            let slice = &self.data[ix..ix + window];
            found = self.all_different(slice, window);
            if !found {
                ix += 1;
            }
        }
        ix + window
    }
}
fn part1(s: &str) {
    let mut s = Stream::new(s);
    let ix = s.process(4);
    println!("Part 1: {ix}");
}
fn part2(s: &str) {
    let mut s = Stream::new(s);
    let ix = s.process(14);
    println!("Part 2: {ix}");
}
pub(crate) fn run() {
    let s = load_str!("../data/day6.txt");
    part1(s);
    part2(s);
}
#[cfg(test)]
mod tests {
    use crate::day6::Stream;
    use load_file::load_str;

    #[test]
    fn samples_part2() {
        let mut s = Stream::new("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        let x = s.process(14);
        assert_eq!(19, x);
        let mut s = Stream::new("bvwbjplbgvbhsrlpgdmjqwftvncz");
        let x = s.process(14);
        assert_eq!(23, x);
    }
    #[test]
    fn samples_part1() {
        //
        //bvwbjplbgvbhsrlpgdmjqwftvncz: first marker after character 5
        // nppdvjthqldpwncqszvftbrmjlhg: first marker after character 6
        // nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg: first marker after character 10
        // zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw: first marker after character 11
        let mut s = Stream::new("bvwbjplbgvbhsrlpgdmjqwftvncz");
        let x = s.process(4);
        assert_eq!(5, x);
        let mut s = Stream::new("nppdvjthqldpwncqszvftbrmjlhg");
        let x = s.process(4);
        assert_eq!(6, x);

        let mut s = Stream::new("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        let x = s.process(4);
        assert_eq!(10, x);

        let mut s = Stream::new("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        let x = s.process(4);
        assert_eq!(11, x);
    }
}
