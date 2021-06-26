use proconio::input;

fn main() {
    input! {
        a: char,
        b: char,
    }

    if (a == 'H' && b == 'H') || (a == 'D' && b == 'D') {
        println!("H");
    } else {
        println!("D");
    }
}
