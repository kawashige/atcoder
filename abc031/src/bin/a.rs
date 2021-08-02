use proconio::input;

fn main() {
    input! {
        a: usize,
        d: usize
    }

    println!("{}", ((a + 1) * d).max(a * (d + 1)));
}
