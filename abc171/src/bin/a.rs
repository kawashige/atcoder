use proconio::input;

fn main() {
    input! {
        a: char
    }

    if a.is_ascii_lowercase() {
        println!("a");
    } else {
        println!("A");
    }
}
