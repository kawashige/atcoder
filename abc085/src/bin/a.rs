use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars
    }

    s[3] = '8';
    println!("{}", s.into_iter().collect::<String>());
}
