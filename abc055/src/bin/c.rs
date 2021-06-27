use proconio::input;

fn main() {
    input! {
        n: u64,
        m: u64
    }

    let s_n = std::cmp::min(n, m / 2);
    let c_n = (m - s_n * 2) / 4;

    println!("{}", s_n + c_n);
}
