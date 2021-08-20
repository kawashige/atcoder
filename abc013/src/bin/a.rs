use proconio::input;

fn main() {
    input! {
        c: char
    }

    println!("{}", 1 + c as u8 - b'A');
}
