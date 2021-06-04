use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[u64; 6]; n]
    }

    let mut r: u64 = 1;
    for i in 0..n {
        r *= a[i].iter().sum::<u64>();
        r %= 1_000_000_007;
    }

    println!("{}", r);
}
