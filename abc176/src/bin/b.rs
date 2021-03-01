use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: Chars
    }

    if n.into_iter()
        .map(|c| c.to_digit(10).unwrap() as u128)
        .sum::<u128>()
        % 9
        == 0
    {
        println!("Yes");
    } else {
        println!("No");
    }
}
