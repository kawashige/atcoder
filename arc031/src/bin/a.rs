use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    if (0..(s.len() / 2)).all(|i| s[i] == s[s.len() - 1 - i]) {
        println!("YES");
    } else {
        println!("NO");
    }
}
