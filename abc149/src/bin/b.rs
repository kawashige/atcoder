use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
        k: u64
    }

    if a + b <= k {
        println!("0 0");
    } else if a < k {
        println!("0 {}", b - (k - a));
    } else {
        println!("{} {}", a - k, b);
    }
}
