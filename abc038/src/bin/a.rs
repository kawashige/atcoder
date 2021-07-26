use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    if s.last() == Some(&'T') {
        println!("YES");
    } else {
        println!("NO");
    }
}
