use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    let mut j = 0;
    for c in "AKIHABARA".chars() {
        if j >= s.len() {
            if c != 'A' {
                println!("NO");
                return;
            } else {
                continue;
            }
        }
        if s[j] != c {
            if c != 'A' {
                println!("NO");
                return;
            }
        } else {
            j += 1;
        }
    }

    if j == s.len() {
        println!("YES");
    } else {
        println!("NO");
    }
}
