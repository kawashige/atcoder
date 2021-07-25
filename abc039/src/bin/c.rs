use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    let mut key = "WBWBWWBWBWBW".chars().collect::<Vec<_>>();
    let r = vec!["Do", "Re", "Mi", "Fa", "So", "La", "Si"];

    for i in 0..r.len() {
        if s.starts_with(&key) {
            println!("{}", r[i]);
            return;
        }

        key.rotate_left(1);
        while key[0] == 'B' {
            key.rotate_left(1);
        }
    }
}
