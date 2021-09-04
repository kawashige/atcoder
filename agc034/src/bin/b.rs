use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    let mut a_count = vec![0_usize; s.len()];
    let mut r = 0;

    for i in 0..(s.len() - 1) {
        if s[i] == 'A' {
            a_count[i] += 1;
            if i > 0 {
                a_count[i] += a_count[i - 1];
            }
        } else if s[i] == 'B' && s[i + 1] == 'C' {
            if i > 0 {
                r += a_count[i - 1];
                a_count[i + 1] += a_count[i - 1];
            }
        }
    }

    println!("{}", r);
}
