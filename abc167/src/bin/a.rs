use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        t: Chars
    }

    if s.len() + 1 == t.len()
        && (0..s.len()).all(|i| s[i] == t[i])
        && t.last().unwrap().is_ascii_lowercase()
    {
        println!("Yes");
    } else {
        println!("No");
    }
}
