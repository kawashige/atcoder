use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let mut x = ((2 * n) as f64).sqrt() as usize;
    while x * (x + 1) < 2 * n {
        x += 1;
    }

    println!("{}", x);
}
