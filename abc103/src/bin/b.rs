use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars,
        t: Chars
    }

    for _ in 0..s.len() {
        s.rotate_right(1);
        if s == t {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
