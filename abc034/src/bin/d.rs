use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut wp: [(f64, f64); n]
    }

    let mut ok = 0.0;
    let mut ng = 100.0;

    for _ in 0..100 {
        let mid = (ok + ng) / 2.0;

        wp.sort_unstable_by(|a, b| {
            (b.0 * (b.1 - mid))
                .partial_cmp(&(a.0 * (a.1 - mid)))
                .unwrap()
        });

        let (_, p) = wp.iter().take(k).fold((0.0, 0.0), |(w, p), (w1, p1)| {
            (w + w1, (w * p + w1 * p1) / (w + w1))
        });
        if p < mid {
            ng = mid
        } else {
            ok = mid
        }
    }

    println!("{}", ok);
}
