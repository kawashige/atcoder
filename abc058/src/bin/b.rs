use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        o: Chars,
        s: Chars,
    }

    let mut r = String::new();
    for i in 0..o.len() {
        r.push(o[i]);
        if i < s.len() {
            r.push(s[i]);
        }
    }

    println!("{}", r);
}
