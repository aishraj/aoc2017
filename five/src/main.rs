use std::fs::File;
use std::io::BufReader;
use std::io::Read;

fn main() {
    let mut arr = read_into_vec("./input.txt");
    let arr_size = arr.len();
    let mut i = 0;
    let mut c = 0;
    while ( i < arr_size || i < 0) {
        let t = arr[i];
       // arr[i] = arr[i] + 1;
        if t < 0 {
            arr[i] = arr[i] + 1;
            let t_value = t.abs();
            let t_value = t_value as usize;
            i = i - t_value;
        } else {
            if t >= 3 {
                arr[i] = arr[i] - 1;
            } else {
                arr[i] = arr[i] + 1;
            }
            let t_value = t as usize;
            i = i + t_value;
        }
        c += 1;
    }
    //println!("Array is {:?}", arr);
    println!("Number of steps is: {:?}", c);
}


fn read_into_vec(filename: &str) -> Vec<i32> {
    let f = File::open(filename).expect("Unable to open the file");
    let mut reader = BufReader::new(f);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).expect(
        "Unable to read file to string",
    );
    let items: Vec<i32> = contents
        .split_whitespace()
        .map(|num| num.parse::<i32>().unwrap())
        .collect();
    items
}
