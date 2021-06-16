use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars
    }

    if s.len() % 2 == 1 {
        s.pop();
    } else {
        s.pop();
        s.pop();
    }

    while !s.is_empty() {
        if s[0..(s.len() / 2)] == s[(s.len() / 2)..] {
            println!("{}", s.len());
            return;
        }
        s.pop();
        s.pop();
    }
}
