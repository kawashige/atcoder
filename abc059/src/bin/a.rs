use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s1: Chars,
        s2: Chars,
        s3: Chars,
    }

    print!(
        "{}{}{}",
        s1[0].to_ascii_uppercase(),
        s2[0].to_ascii_uppercase(),
        s3[0].to_ascii_uppercase()
    )
}
