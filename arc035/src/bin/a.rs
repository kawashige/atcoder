use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    for i in 0..=(s.len() / 2) {
        if s[i] != s[s.len() - 1 - i] && (s[i] != '*' && s[s.len() - 1 - i] != '*') {
            println!("NO");
            return;
        }
    }
    println!("YES");
}
