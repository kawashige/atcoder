use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    let minus = s.iter().filter(|c| c == &&'-').count();
    println!("{}", 4 - minus as i32 * 2);
}
