use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        t: Chars
    }

    let mut max = 0;
    for i in 0..=(s.len() - t.len()) {
        let mut count = 0;
        for j in 0..t.len() {
            if s[i + j] == t[j] {
                count += 1;
            }
        }
        max = std::cmp::max(max, count);
    }

    println!("{}", t.len() - max);
}
