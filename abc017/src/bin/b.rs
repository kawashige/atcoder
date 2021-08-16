use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut x: Chars
    }

    while !x.is_empty() {
        match x.last() {
            Some('o') | Some('k') | Some('u') => {
                x.pop();
            }
            Some('h') => {
                x.pop();
                if x.last() == Some(&'c') {
                    x.pop();
                } else {
                    println!("NO");
                    return;
                }
            }
            _ => {
                println!("NO");
                return;
            }
        }
    }

    println!("YES");
}
