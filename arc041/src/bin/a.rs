use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
        k: usize
    }

    if k <= y {
        println!("{}", x + k);
    } else {
        println!("{}", y + x - (k - y));
    }
}
