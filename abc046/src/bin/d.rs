use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    let mut c = 0;
    let mut p = 0;
    for i in 0..s.len() {
        if s[i] == 'g' {
            if c > 0 {
                p += 1;
                c -= 1;
            } else {
                c += 1;
            }
        } else {
            if c > 0 {
                c -= 1;
            } else {
                p -= 1;
                c += 1;
            }
        }
    }

    println!("{}", p);
}
