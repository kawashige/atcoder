use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        a: Chars
    }

    let mut count = 0;
    let mid = a.len() / 2 + if a.len() % 2 == 1 { 1 } else { 0 };
    for i in 0..mid {
        if a[i] != a[a.len() - 1 - i] {
            count += 1;
        }
    }

    let mut r = 0;
    for i in 0..mid {
        if i == a.len() - 1 - i {
            if count != 0 {
                r += 25;
            }
        } else if a[i] != a[a.len() - 1 - i] {
            if count == 1 {
                r += 48;
            } else {
                r += 50;
            }
        } else {
            r += 50;
        }
    }

    println!("{}", r);
}
