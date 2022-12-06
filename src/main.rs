#[macro_use]
extern crate lazy_static;
lazy_static! {
    pub static ref LOWER: String = "abcdefghijklmnopqrstuvwxyz".to_string();
    pub static ref UPPER: String = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();
}

extern crate core;
extern crate load_file;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    // day1::run();
    // day2::run();
    // day2::run_threaded();
    // day3::run();
    // day4::run();
    //day5::run();
    day6::run();
}
