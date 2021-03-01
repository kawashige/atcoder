use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    if s[0] == 'R' && s[1] == 'R' && s[2] == 'R' {
        println!("3");
    } else if (s[0] == 'R' && s[1] == 'R') || (s[1] == 'R' && s[2] == 'R') {
        println!("2");
    } else if s[0] == 'R' || s[1] == 'R' || s[2] == 'R' {
        println!("1");
    } else {
        println!("0");
    }
}
