use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize
    }

    println!("{}", (1900 * m + 100 * (n - m)) * 2_usize.pow(m as u32));
}
