use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars
    }

    let count = (0..(n - 2))
        .filter(|i| s[*i] == 'A' && s[i + 1] == 'B' && s[i + 2] == 'C')
        .count();
    println!("{}", count);
}
