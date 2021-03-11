use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        n: usize
    }

    if b == 1 {
        println!("0");
    } else if n < b {
        println!("{}", (a * n) / b);
    } else if n == b {
        println!("{}", (a * (n - 1)) / b);
    } else {
        println!("{}", std::cmp::max(a / b, (a * (b - 1)) / b));
    }
}
