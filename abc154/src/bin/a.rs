use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
        a: usize,
        b: usize,
        u: String
    }

    if u == s {
        println!("{} {}", a - 1, b);
    } else {
        println!("{} {}", a, b - 1);
    }
}
