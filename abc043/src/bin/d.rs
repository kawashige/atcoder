use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    for i in 1..s.len() {
        if s[i - 1] == s[i] {
            println!("{} {}", i, i + 1);
            return;
        } else if i > 1 && s[i - 2] == s[i] {
            println!("{} {}", i - 1, i + 1);
            return;
        }
    }

    println!("-1 -1");
}
