use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        k: usize
    }

    let mut l = 0;
    let mut c = 0;
    let mut r = 0;

    for i in 0..s.len() {
        if s[i] == '.' {
            c += 1;
            while k < c {
                if s[l] == '.' {
                    c -= 1;
                }
                l += 1;
            }
        }
        r = r.max(i - l + 1);
    }

    println!("{}", r);
}
