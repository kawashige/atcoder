use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        x: usize,
        y: usize
    }

    if n < k {
        println!("{}", x * n);
    } else {
        println!("{}", x * k + y * (n - k));
    }
}
