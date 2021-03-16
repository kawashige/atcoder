use proconio::input;

fn main() {
    input! {
        l: f64
    }

    let n = l / 3.0;
    let r = n * n * (l - 2.0 * n);

    println!("{}", r);
}
