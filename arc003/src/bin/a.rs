use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        r: Chars
    }

    let s = r
        .into_iter()
        .map(|c| match c {
            'A' => 4,
            'B' => 3,
            'C' => 2,
            'D' => 1,
            _ => 0,
        })
        .sum::<usize>() as f64
        / n as f64;

    println!("{}", s);
}
