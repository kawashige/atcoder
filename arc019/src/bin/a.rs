use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    let r = s
        .into_iter()
        .map(|i| match i {
            'O' => '0',
            'D' => '0',
            'I' => '1',
            'Z' => '2',
            'S' => '5',
            'B' => '8',
            _ => i,
        })
        .collect::<String>();

    println!("{}", r);
}
