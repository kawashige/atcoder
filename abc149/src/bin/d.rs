use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        k: usize,
        r: u64,
        s: u64,
        p: u64,
        t: Chars
    }

    let mut points = Vec::with_capacity(n);
    for i in 0..n {
        points.push(if i >= k && t[i - k] == t[i] && points[i - k] > 0 {
            0
        } else {
            match t[i] {
                'r' => p,
                's' => r,
                'p' => s,
                _ => unreachable!(),
            }
        });
    }

    let sum: u64 = points.into_iter().sum();

    println!("{}", sum);
}
