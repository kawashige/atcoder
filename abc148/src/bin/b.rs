use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars
    }

    let mut r = String::new();
    for (c_s, c_t) in s.into_iter().zip(t.into_iter()) {
        r.push(c_s);
        r.push(c_t);
    }
    println!("{}", r);
}
