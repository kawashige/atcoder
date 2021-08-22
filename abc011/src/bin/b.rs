use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars
    }

    for i in 0..s.len() {
        if i == 0 {
            s[0] = s[0].to_ascii_uppercase()
        } else {
            s[i] = s[i].to_ascii_lowercase();
        }
    }

    println!("{}", s.into_iter().collect::<String>());
}
