use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let r = (n as f64).sqrt() as usize;
    println!("{}", r * r);
}
