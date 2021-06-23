use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        a: Chars,
        b: Chars,
    }

    if a.len() > b.len() {
        println!("GREATER");
    } else if a.len() < b.len() {
        println!("LESS");
    } else {
        for i in 0..a.len() {
            if a[i] > b[i] {
                println!("GREATER");
                return;
            } else if a[i] < b[i] {
                println!("LESS");
                return;
            }
        }
        println!("EQUAL");
    }
}
