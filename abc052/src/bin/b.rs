use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: usize,
        s: Chars
    }

    let mut x = 0;
    let mut max = 0;
    for c in s {
        if c == 'I' {
            x += 1;
            max = std::cmp::max(max, x);
        } else {
            x -= 1;
        }
    }

    println!("{}", max);
}
