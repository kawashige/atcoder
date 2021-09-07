use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    let mut l = 0;
    let mut r = s.len() - 1;

    let mut count = 0;

    while l < r {
        if s[l] == 'x' && s[r] != 'x' {
            count += 1;
            l += 1;
        } else if s[l] != 'x' && s[r] == 'x' {
            count += 1;
            r -= 1;
        } else if s[l] != 'x' && s[r] != 'x' && s[l] != s[r] {
            println!("-1");
            return;
        } else {
            l += 1;
            r -= 1;
        }
    }

    println!("{}", count);
}
