use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars,
        t: Chars
    }

    for i in 0..(s.len() - 1) {
        if s[i] != t[i] {
            s.swap(i, i + 1);
            break;
        }
    }

    if s == t {
        println!("Yes");
    } else {
        println!("No");
    }
}
