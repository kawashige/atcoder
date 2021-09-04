use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: usize,
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        s: Chars,
    }

    if (a..c).all(|i| s[i - 1] == '.' || s[i] == '.')
        && (b..d).all(|i| s[i - 1] == '.' || s[i] == '.')
        && c < d
        || (b.max(2)..=d).any(|i| s[i - 2] == '.' && s[i - 1] == '.' && s[i] == '.')
    {
        println!("Yes");
    } else {
        println!("No");
    }
}
