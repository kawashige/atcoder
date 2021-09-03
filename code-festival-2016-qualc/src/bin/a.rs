use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    if let Some(i) = (0..s.len()).find(|i| s[*i] == 'C') {
        if let Some(_) = ((i + 1)..s.len()).find(|i| s[*i] == 'F') {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
