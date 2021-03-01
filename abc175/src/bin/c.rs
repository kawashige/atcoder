use proconio::input;

fn main() {
    input! {
        x: i128,
        k: i128,
        d: i128
    }

    let x = x.abs();
    let max = x / d;
    if k <= max {
        println!("{}", x - k * d);
    } else if (k - max) % 2 == 0 {
        println!("{}", x - max * d);
    } else {
        println!("{}", (x - max * d - d).abs());
    }
}
