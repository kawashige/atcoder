use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let n = (1..n).fold(k, |acc, _| acc * (k - 1));
    println!("{}", n);
}
