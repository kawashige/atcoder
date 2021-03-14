use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        mut w: [(u128, u128); n],
        x: [u128; m],
        q: [(usize, usize); q]
    }

    w.sort_unstable_by(|a, b| b.1.cmp(&a.1));
    let mut indices = x.into_iter().enumerate().collect::<Vec<(usize, u128)>>();
    indices.sort_unstable_by_key(|(_, x)| *x);

    for (f, t) in q {
        let mut sum: u128 = 0;
        let mut used = vec![false; m];
        for (w, v) in &w {
            for i in 0..m {
                if (f..=t).contains(&(indices[i].0 + 1)) || &indices[i].1 < w || used[i] {
                    continue;
                }
                sum += v;
                used[i] = true;
                break;
            }
        }
        println!("{}", sum);
    }
}
