use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        k: usize,
        s: usize,
        t: usize
    }

    println!("{}", a * s + b * t - c * if s + t < k { 0 } else { s + t });
}
