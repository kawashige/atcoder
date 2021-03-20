use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize
    }

    if (a == b && a != c) || (a == c && a != b) || (b == c && b != a) {
        println!("Yes");
    } else {
        println!("No");
    }
}
