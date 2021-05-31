use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars,
        mut t: Chars
    }

    s.sort_unstable();
    if t.iter().any(|c| c > &s[0])
        || (s.iter().all(|c| c == &s[0]) && t.iter().all(|c| c == &s[0]) && s.len() < t.len())
    {
        println!("Yes");
    } else {
        println!("No");
    }
}
