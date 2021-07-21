use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        w: usize
    }

    println!(
        "{}",
        (0..s.len()).step_by(w).map(|i| s[i]).collect::<String>()
    )
}
