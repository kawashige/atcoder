use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
    }

    const M: u64 = 1_000_000_007;

    println!("{}", (a * b % M) * c % M);
}
