use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("./input.txt").expect("Unable to open file");
    let f = BufReader::new(f);
    let mut running_total = 0; 
    for line in f.lines() {
        let line = line.expect("Unable to read line");
        let items: Vec<_> = line.split("\t").map(|num| num.parse::<i32>().unwrap()).collect();
        let max_item = items.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let min_item = items.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        println!("Items are {:?} and {:?}", max_item, min_item);
        let diff = max_item - min_item;
        println!("Current diff is {:?}", diff);
        running_total = running_total + diff;
    }
    println!("Aggregate diff is {:?}", running_total);
}