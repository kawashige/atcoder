use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars
    }

    if n % 2 == 0
        && s[..(n / 2)].iter().collect::<String>() == s[(n / 2)..].iter().collect::<String>()
    {
        println!("Yes");
    } else {
        println!("No");
    }
}
