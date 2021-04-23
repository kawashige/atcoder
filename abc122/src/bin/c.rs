use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        lr: [(usize, usize); q]
    }

    let mut counts = vec![0; n + 1];
    for i in 1..n {
        counts[i + 1] = counts[i];
        if s[i - 1] == 'A' && s[i] == 'C' {
            counts[i + 1] += 1;
        }
    }

    for (l, r) in lr {
        println!("{}", counts[r] - counts[l])
    }
}
