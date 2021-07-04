use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: [Chars; 3]
    }

    println!("{}{}{}", s[0][0], s[1][0].to_ascii_uppercase(), s[2][0]);
}
