use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        a: Chars
    }

    let mut count = [0; 26];
    let mut r = 1;
    for i in (0..a.len()).rev() {
        r += (a.len() - 1 - i) - count[a[i] as usize - 0x61];
        count[a[i] as usize - 0x61] += 1;
    }

    println!("{}", r);
}
