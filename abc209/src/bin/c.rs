use proconio::input;

fn main() {
    input! {
        n: usize,
        mut c: [u64; n]
    }
    let m = 1_000_000_007;

    c.sort_unstable();

    let mut count = c[0];
    for i in 1..n {
        if c[i] < i as u64 + 1 {
            println!("0");
            return;
        }
        count *= c[i] - i as u64;
        count %= m;
    }

    println!("{}", count);
}
