use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    let mut r = String::new();

    for c in s {
        match c {
            'B' if !r.is_empty() => {
                r.pop();
            }
            'B' => {}
            _ => {
                r.push(c);
            }
        }
    }

    println!("{}", r);
}
