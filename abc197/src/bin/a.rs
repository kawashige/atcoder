use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    println!("{}{}{}", s[1], s[2], s[0]);
}
