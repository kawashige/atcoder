use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    let l = (0..s.len()).find(|i| s[*i] == 'A');
    let r = (0..s.len()).rev().find(|i| s[*i] == 'Z');

    let r = match (l, r) {
        (Some(i), Some(j)) => {
            if i > j {
                0
            } else {
                j - i + 1
            }
        }
        _ => 0,
    };

    println!("{}", r);
}
