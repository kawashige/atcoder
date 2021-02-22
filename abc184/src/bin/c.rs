use proconio::input;

fn main() {
    input! {
        r1: i32,
        c1: i32,
        r2: i32,
        c2: i32,
    }

    let count = match ((r1 - r2).abs(), (c1 - c2).abs()) {
        (0, 0) => 0,
        (x, y) if x + y <= 3 => 1,
        (x, y) if x == y => 1,
        (x, y) if x + y <= 6 || (x - y).abs() <= 3 => 2,
        (x, y) if (x + y) % 2 == 0 => 2,
        _ => 3,
    };

    println!("{}", count);
}
