use proconio::input;

fn main() {
    input! {
        q: u64,
        h: u64,
        s: u64,
        d: u64,
        n: u64,
    }

    let one = (q * 4).min(h * 2).min(s);
    let two = (one * 2).min(d);

    println!("{}", (n / 2) * two + (n % 2) * one);
}
