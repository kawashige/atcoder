use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        l: usize,
        r: usize,
        s: Chars
    }

    println!(
        "{}{}{}",
        s[..(l - 1)].iter().collect::<String>(),
        s[(l - 1)..r].iter().rev().collect::<String>(),
        s[r..].iter().collect::<String>()
    )
}
