use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
    }

    println!("{} {}", x.min(y), if x + y <= n { 0 } else { x + y - n });
}
