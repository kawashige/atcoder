use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    let r = 700 + s.into_iter().filter(|c| c == &'o').count() * 100;

    println!("{}", r);
}
