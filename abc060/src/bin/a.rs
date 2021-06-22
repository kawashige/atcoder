use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        a: Chars,
        b: Chars,
        c: Chars,
    }

    if a.last().unwrap() == &b[0] && b.last().unwrap() == &c[0] {
        println!("YES");
    } else {
        println!("NO");
    }
}
