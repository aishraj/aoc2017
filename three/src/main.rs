fn main() {
    let target = 277678;
    println!("{}", solve(target));
}

fn solve(target: i32) -> i32 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut num: i32 = 1;
    let mut d: i32 = 1;
    loop {
        y += d;
        num += d;
        if num >= target {
            y -= num - target;
            break;
        }
        x -= d;
        num += d;
        if num >= target {
            x += num - target;
            break;
        }
        d += 1;
        y -= d;
        num += d;
        if num >= target {
            y += num - target;
            break;
        }
        x += d;
        num += d;
        if num >= target {
            x -= num - target;
            break;
        }
        d += 1;
    }
    x.abs() + y.abs()
}
