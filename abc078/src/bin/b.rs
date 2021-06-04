use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
        z: usize
    }

    let r = (x - z) / (y + z);
    println!("{}", r);
}
