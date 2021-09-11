use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        x: Chars
    }

    let mut stack = Vec::with_capacity(x.len());

    for c in x {
        if c == 'T' && stack.last() == Some(&'S') {
            stack.pop();
        } else {
            stack.push(c);
        }
    }

    println!("{}", stack.len());
}
