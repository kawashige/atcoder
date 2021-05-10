use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize
    }

    if n % k == 0 {
        println!("0");
    } else {
        println!("1");
    }
}
