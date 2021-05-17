use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars
    }

    let mut counts = vec![vec![0; 26]; n];
    let mut count = vec![0; 26];
    for i in 0..n {
        count[s[i] as usize - 0x61] += 1;
        counts[i] = count.clone();
    }

    let mut max = 0;
    for i in 0..(n - 1) {
        let count = (0..26)
            .filter(|j| counts[i][*j] > 0 && counts[n - 1][*j] > counts[i][*j])
            .count();
        max = std::cmp::max(max, count);
    }

    println!("{}", max);
}
