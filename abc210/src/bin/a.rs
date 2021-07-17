use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        x: usize,
        y: usize
    }

    if a < n {
        println!("{}", x * a + y * (n - a));
    } else {
        println!("{}", x * n);
    }
}
