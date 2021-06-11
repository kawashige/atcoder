use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s1: Chars,
        s2: Chars
    }

    let mut r = if s1[0] == s2[0] { 3 } else { 6 } as u64;
    let mut i = if s1[0] == s2[0] { 1 } else { 2 };

    while i < n {
        if s1[i - 1] == s2[i - 1] {
            r *= 2;
            r %= 1_000_000_007;
        }
        if s1[i - 1] != s2[i - 1] && s1[i] != s2[i] {
            r *= 3;
            r %= 1_000_000_007;
        }

        if s1[i] == s2[i] {
            i += 1;
        } else {
            i += 2;
        }
    }

    println!("{}", r);
}
