use proconio::input;

fn main() {
    input! {
        n: usize,
        a: u64,
        b: u64,
        x: [u64; n]
    }

    let mut r = 0;
    for i in 1..n {
        if (x[i] - x[i - 1]) * a < b {
            r += (x[i] - x[i - 1]) * a;
        } else {
            r += b;
        }
    }

    println!("{}", r);
}
