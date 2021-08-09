use proconio::input;

fn main() {
    input! {
        n: usize,
        hs: [(u64, u64); n]
    }

    let max = hs
        .iter()
        .map(|(h, s)| h + s * (n - 1) as u64)
        .max()
        .unwrap();

    let mut ng = 0;
    let mut ok = max;

    while ng + 1 < ok {
        let mid = (ok + ng) / 2;

        let mut nums = hs
            .iter()
            .map(|(h, s)| if h > &mid { -1 } else { ((mid - h) / s) as i64 })
            .collect::<Vec<_>>();
        nums.sort_unstable();

        if (0..n).any(|i| nums[i] < i as i64) {
            ng = mid;
        } else {
            ok = mid;
        }
    }

    println!("{}", ok);
}
