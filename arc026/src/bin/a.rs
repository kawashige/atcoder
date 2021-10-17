use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }

    let r = if n > 5 { a * (n - 5) + b * 5 } else { n * b };
    println!("{}", r);
}
