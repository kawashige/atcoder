use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    if s.len() == 1 {
        println!("Yes");
        return;
    }

    let pattern = match (s[0], s[1]) {
        ('o', 'x') => ['o', 'x', 'x'],
        ('x', 'x') => ['x', 'x', 'o'],
        ('x', 'o') => ['x', 'o', 'x'],
        _ => ['?', '?', '?'],
    };

    if (0..s.len()).all(|i| pattern[i % 3] == s[i]) {
        println!("Yes");
    } else {
        println!("No");
    }
}
