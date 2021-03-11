use proconio::input;

fn main() {
    input! {
        k: usize,
        a: usize,
        b: usize
    }

    if (a..=b).any(|n| n % k == 0) {
        println!("OK");
    } else {
        println!("NG");
    }
}
