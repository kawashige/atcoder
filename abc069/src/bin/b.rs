use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    println!("{}{}{}", s[0], s.len() - 2, s.last().unwrap());
}
