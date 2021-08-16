use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: usize,
        s: Chars
    }

    let r = s
        .into_iter()
        .map(|c| c as usize - b'a' as usize)
        .rev()
        .fold(0, |acc, x| acc * 2 + x);

    println!("{}", r);
}
