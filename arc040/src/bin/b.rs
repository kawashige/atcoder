use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        r: usize,
        s: Chars
    }

    let mut t = 0;
    let mut painted = -1;
    if let Some(j) = (0..n).rev().find(|j| s[*j] == '.') {
        for i in 0..=j {
            if (painted < i as i32 && s[i] == '.') || j <= i + r - 1 {
                painted = (i + r - 1) as i32;
                t += 1;
            }
            if painted >= j as i32 {
                break;
            }
            t += 1;
        }
    }
    println!("{}", t);
}
