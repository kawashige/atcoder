use proconio::input;
fn main() {
    input! {
        n: usize,
        m: usize
    }

    let num = if n == 0 { 0 } else { n * (n - 1) / 2 } + if m == 0 { 0 } else { m * (m - 1) / 2 };

    println!("{}", num);
}
