use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u64,
        a: [u64; n]
    }

    let mut sum: u64 = 0;
    let mut count: u64 = 0;
    let mut s = 0;
    for i in 0..n {
        sum += a[i];
        if sum < k {
            continue;
        }
        while sum.saturating_sub(a[s]) >= k {
            sum -= a[s];
            s += 1;
        }
        count += s as u64 + 1;
    }

    println!("{}", count);
}
