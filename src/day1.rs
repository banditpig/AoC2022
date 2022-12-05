use load_file::load_str;

pub(crate) fn run() {
    let numbers_spaces = load_str!("../data/day1.txt")
        .split('\n')
        .collect::<Vec<&str>>();

    let mut sums_vec = Vec::new();
    let mut sum = 0;
    for ns in numbers_spaces {
        if let Ok(n) = ns.parse::<i32>() {
            sum += n
        } else {
            sums_vec.push(sum);
            sum = 0;
        }
    }
    sums_vec.sort_by(|a, b| b.cmp(a));
    println!("part 1 {}", sums_vec[0]);
    println!("part 2 {}", sums_vec[0] + sums_vec[1] + sums_vec[2]);
}
