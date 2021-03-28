use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    let mut count = 0;
    for i in 0..(s.len() / 2) {
        if s[i] != s[s.len() - 1 - i] {
            count += 1;
        }
    }
    println!("{}", count);
}
