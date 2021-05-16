use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize
    }

    let n = b - a;
    let h = (n * (n + 1)) / 2;

    println!("{}", h - b);
}
