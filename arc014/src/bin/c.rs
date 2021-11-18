use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: usize,
        s: Chars
    }

    let count = s.iter().fold([0; 3], |mut count, c| {
        count[match c {
            'R' => 0,
            'G' => 1,
            _ => 2,
        }] += 1;
        count
    });

    println!("{}", count.iter().map(|c| c % 2).sum::<i32>());
}
