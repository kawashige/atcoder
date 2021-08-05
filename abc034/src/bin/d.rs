use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut wp: [(f64, f64); n]
    }

    wp.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let mut used = vec![false; n];
    used[0] = true;
    let mut w = wp[0].0;
    let mut p = wp[0].1;

    for _ in 0..(k - 1) {
        let mut max_i = 0;
        let mut max_p = 0.0;
        for i in 0..n {
            if used[i] {
                continue;
            }
            let tmp_p = (wp[i].0 * wp[i].1 + w * p) / (wp[i].0 + w);
            if max_p < tmp_p {
                max_p = tmp_p;
                max_i = i;
            }
        }

        used[max_i] = true;
        w += wp[max_i].0;
        p = max_p;
    }

    println!("{}", p);
}
