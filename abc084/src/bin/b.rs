use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        a: usize,
        b: usize,
        s: Chars
    }

    if (0..(a + b + 1)).all(|i| (i == a && s[i] == '-') || (i != a && s[i].is_numeric())) {
        println!("Yes");
    } else {
        println!("No");
    }
}
