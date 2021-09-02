use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        a: Chars,
        b: Chars,
        c: Chars,
    }

    let mut r = 0;
    for i in 0..n {
        r += if a[i] != b[i] && b[i] != c[i] && a[i] != c[i] {
            2
        } else if a[i] == b[i] && a[i] == c[i] && b[i] == c[i] {
            0
        } else {
            1
        };
    }

    println!("{}", r);
}
