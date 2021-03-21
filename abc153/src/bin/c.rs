use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u64,
        mut h: [u64; n]
    }

    h.sort_unstable();

    if n as u64 <= k {
        println!("0");
    } else {
        println!("{}", h.iter().take(n - k as usize).sum::<u64>());
    }
}
