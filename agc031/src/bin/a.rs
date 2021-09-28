use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: usize,
        s: Chars
    }

    let count = s.into_iter().fold([0_usize; 26], |mut count, c| {
        count[c as usize - 0x61] += 1;
        count
    });

    let r = count
        .iter()
        .filter(|x| x > &&0)
        .fold(1_usize, |acc, c| (c + 1) * acc % 1_000_000_007)
        - 1;

    println!("{}", r);
}
