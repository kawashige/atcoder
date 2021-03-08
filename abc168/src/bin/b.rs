use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        k: usize,
        s: Chars
    }

    if s.len() > k {
        println!("{}...", s.iter().take(k).collect::<String>());
    } else {
        println!("{}", s.iter().collect::<String>());
    }
}
