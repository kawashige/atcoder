use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars
    }

    let i = (0..n).find(|i| s[*i] == '1').unwrap();

    if i % 2 == 0 {
        println!("Takahashi");
    } else {
        println!("Aoki");
    }
}
