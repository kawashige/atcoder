use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    if (a == 1 && b > 1) || (a != 1 && b != 1 && a > b) {
        println!("Alice");
    } else if (b == 1 && a > 1) || (a != 1 && b != 1 && b > a) {
        println!("Bob");
    } else {
        println!("Draw");
    }
}
