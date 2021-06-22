use proconio::input;

fn main() {
    input! {
        n: usize,
        time: u64,
        t: [u64; n]
    }

    let mut r = time * n as u64;
    for i in 1..n {
        if t[i] - t[i - 1] < time {
            r -= time - (t[i] - t[i - 1]);
        }
    }

    println!("{}", r);
}
