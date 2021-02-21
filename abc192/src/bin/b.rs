use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        chars: Chars
    }

    let chars: Vec<char> = chars;

    for (i, c) in chars.into_iter().enumerate() {
        if (i + 1) % 2 == 0 && c.is_uppercase() {
            continue;
        } else if (i + 1) % 2 == 1 && c.is_lowercase() {
            continue;
        }

        println!("No");
        return;
    }
    println!("Yes");
}
