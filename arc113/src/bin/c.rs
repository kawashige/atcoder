use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    let mut count = [0; 26];
    let mut r = 0;

    for i in (1..s.len()).rev() {
        if i < s.len() - 1 && s[i - 1] == s[i] && s[i] != s[i + 1] {
            r += s.len() - 1 - i - count[s[i] as usize - 0x61];

            count = [0; 26];
            count[s[i] as usize - 0x61] = s.len() - i;
        } else {
            count[s[i] as usize - 0x61] += 1;
        }
    }

    println!("{}", r);
}
