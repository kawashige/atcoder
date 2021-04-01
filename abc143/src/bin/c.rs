use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars
    }

    let mut count = 1;
    for i in 1..n {
        if s[i - 1] != s[i] {
            count += 1;
        }
    }

    println!("{}", count);
}
