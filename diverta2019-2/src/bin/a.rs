use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize
    }

    if k == 1 {
        println!("0");
    } else {
        println!("{}", n - k);
    }
}
