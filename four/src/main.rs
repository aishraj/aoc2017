use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    println!("Number of valid pass phrases: {:?}", solve_part_one());
    println!("Number of valid pass phrases: {:?}", solve_part_two());
}

fn solve_part_one() -> i32 {
    let f = File::open("./input.txt").expect("Unable to open the file");
    let f = BufReader::new(f);
    let mut running_total :i32 = 0;
    for line in f.lines() {
        let line_item = line.expect("Unable to read line");
        let items: Vec<&str> = line_item.split_whitespace().collect();
        let vec_len = items.len();
        let uniques: HashSet<&str> = HashSet::from_iter(items);
        if vec_len == uniques.len() {
            running_total += 1;
        }
    }
    return running_total;
}

fn solve_part_two() -> i32 {
    return 0;
}
