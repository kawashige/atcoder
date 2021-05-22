use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    let r = s
        .into_iter()
        .rev()
        .map(|c| match c {
            '0' | '1' | '8' => c,
            '6' => '9',
            '9' => '6',
            _ => unreachable!(),
        })
        .collect::<String>();
    println!("{}", r);
}
