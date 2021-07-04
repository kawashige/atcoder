use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    if ((s.len() % 2 == 1 && &s[0] != s.last().unwrap())
        || (s.len() % 2 == 0 && &s[0] == s.last().unwrap()))
        && (1..(s.len() - 1)).any(|i| s[i - 1] != s[i + 1])
    {
        println!("First");
    } else {
        println!("Second");
    }
}
