use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    if s.windows(2).any(|c| c == ['A', 'C']) {
        println!("Yes");
    } else {
        println!("No");
    }
}
