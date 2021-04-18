use proconio::input;

fn main() {
    input! {
        r: usize,
        d: usize,
        x: usize
    }

    let mut x = x;
    for _ in 0..10 {
        x = x * r - d;
        println!("{}", x);
    }
}
