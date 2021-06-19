use proconio::input;

fn main() {
    input! {
        n: u64,
        a: u64,
        b: u64,
        mut h: [u64; n as usize]
    }

    h.sort_unstable_by(|a, b| b.cmp(&a));

    let mut ng = 0;
    let mut ok = (h[0] + b - 1) / b;

    while ng + 1 < ok {
        let mid = (ok + ng) / 2;
        let mut count = 0;
        for i in 0..(n as usize) {
            if h[i] > b * mid {
                count += (h[i] - b * mid + a - b - 1) / (a - b);
            }
        }

        if mid < count {
            ng = mid;
        } else {
            ok = mid;
        }
    }

    println!("{}", ok);
}
