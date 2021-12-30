use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    print!(
        "{}",
        s[0].to_digit(10).unwrap() * s[2].to_digit(10).unwrap()
    );
}
