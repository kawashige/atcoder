use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        t: Chars
    }

    println!("{}", (0..s.len()).filter(|i| s[*i] != t[*i]).count())
}
