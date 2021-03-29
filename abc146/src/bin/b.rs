use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: u8,
        s: Chars
    }

    let s = s
        .into_iter()
        .map(|c| (((c as u8 - 0x41 + n) % 26) + 0x41) as char)
        .collect::<String>();

    println!("{}", s);
}
