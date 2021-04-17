use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
        z: usize
    }

    if (z * y) % x == 0 {
        println!("{}", (z * y) / x - 1);
    } else {
        println!("{}", (z * y) / x);
    }
}
