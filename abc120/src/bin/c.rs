use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    let mut count = 0;
    let mut stack = vec![s[0]];
    for i in 1..s.len() {
        if stack.is_empty() || stack.last().unwrap() == &s[i] {
            stack.push(s[i]);
        } else if !stack.is_empty() {
            stack.pop();
            count += 2;
        }
    }

    println!("{}", count);
}
