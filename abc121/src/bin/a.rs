use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        x: usize,
        y: usize
    }

    println!("{}", h * w - (x * w + y * h - x * y));
}
