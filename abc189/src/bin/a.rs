use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        chars: Chars
    }

    if (1..chars.len()).all(|i| chars[i] == chars[0]) {
        println!("Won");
    } else {
        println!("Lost");
    }
}
