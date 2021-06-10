use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    println!("{}", s.into_iter().step_by(2).collect::<String>());
}
