use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _: usize,
        x: usize,
        s: Chars
    }

    let score = s.into_iter().fold(x, |sum, c| {
        if c == 'o' {
            sum + 1
        } else {
            sum.saturating_sub(1)
        }
    });
    println!("{}", score);
}
