use proconio::input;

fn main() {
    input! {
        mut x: u128,
        y: u128,
        a: u128,
        b: u128,
    }

    let mut count = 0;
    let max = b / (a - 1);
    while x * a < y && x * a < max {
        x *= a;
        count += 1;
    }

    if x < y {
        count += (y - x) / b;
        if (y - x) % b == 0 {
            count -= 1;
        }
    }

    println!("{}", count);
}
