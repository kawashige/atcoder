use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars
    }

    s.sort_unstable();

    if "abc" == s.into_iter().collect::<String>() {
        println!("Yes");
    } else {
        println!("No");
    }
}
