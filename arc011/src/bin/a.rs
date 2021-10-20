use proconio::input;

fn main() {
    input! {
        m: usize,
        n: usize,
        nn: usize
    }

    let mut x = nn;
    let mut r = x;
    while x >= m {
        r += (x / m) * n;
        x = (x / m) * n + x % m;
    }

    println!("{}", r);
}
