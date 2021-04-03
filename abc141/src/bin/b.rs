use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    if s.into_iter()
        .enumerate()
        .all(|(i, c)| match ((i + 1) % 2, c) {
            (0, 'L') | (1, 'R') => true,
            (_, 'U') | (_, 'D') => true,
            _ => false,
        })
    {
        println!("Yes");
    } else {
        println!("No");
    }
}
