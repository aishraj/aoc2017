use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("./input.txt").expect("Unable to open file");
    let f = BufReader::new(f);
    let mut running_total = 0; 
    for line in f.lines() {
        let line = line.expect("Unable to read line");
        let items: Vec<_> = line.split_whitespace().map(|num| num.parse::<i32>().unwrap()).collect();
        let max_item = items.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let min_item = items.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        //println!("Items are {:?} and {:?}", max_item, min_item);
        let diff = max_item - min_item;
        //println!("Current diff is {:?}", diff);
        running_total = running_total + diff;
    }
    println!("Aggregate diff is {:?}", running_total);
    println!("Part two's solution is: {:?}", solve_part_two("./test.txt"));
}

fn solve_part_two(filepath: &str) -> i32 {
    let f = File::open(filepath).expect("Unable to open file");
    let f = BufReader::new(f);
    let mut running_total :i32 = 0; 
    for line in f.lines() {
        let line = line.expect("Unable to read line");
        let mut items: Vec<i32> = line.split_whitespace().map(|num| num.parse::<i32>().unwrap()).collect();
        items.sort();
        //Brute for for now
        let item_size = items.len();
        for i in 0..item_size {
            for j in (i+1)..item_size {
                if items[j] % items[i] == 0 {
                    running_total += (items[j] / items[i]);
                    break;
                }
            }
        }
    }
    println!("Aggregate of divisors is {:?}", running_total);
    return running_total;
}