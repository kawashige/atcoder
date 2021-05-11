use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    let n = s.len() - 1;
    if s[0] == 'A'
        && s[1].is_ascii_lowercase()
        && s[n].is_ascii_lowercase()
        && s[2..n].iter().filter(|c| c == &&'C').count() == 1
        && s[2..n].iter().all(|c| c == &'C' || c.is_ascii_lowercase())
    {
        println!("AC");
    } else {
        println!("WA");
    }
}
