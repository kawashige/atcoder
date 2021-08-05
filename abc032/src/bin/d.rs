use proconio::input;

fn dfs(i: usize, v: usize, w: usize, vw: &mut Vec<(usize, usize)>, max: &mut usize) {
    *max = std::cmp::max(*max, v);

    if i >= vw.len() {
        return;
    }

    if vw[i].1 <= w {
        let mut opt_v = vw[i].0;
        let mut opt_w = vw[i].1;
        let mut j = i + 1;

        while j < vw.len() && opt_w + vw[j].1 <= w {
            opt_v += vw[j].0;
            opt_w += vw[j].1;
            j += 1;
        }
        if j < vw.len() && opt_w < w {
            opt_v = (opt_v as f64 + (vw[j].0 as u64 * (w - opt_w) as u64) as f64 / vw[j].1 as f64)
                as usize;
        }

        if *max <= opt_v + v {
            dfs(i + 1, v + vw[i].0, w - vw[i].1, vw, max);
        }
    }

    dfs(i + 1, v, w, vw, max);
}

fn main() {
    input! {
        n: usize,
        w: usize,
        mut vw: [(usize, usize); n]
    }

    vw.sort_unstable_by(|a, b| {
        (b.0 as f64 / b.1 as f64)
            .partial_cmp(&(a.0 as f64 / a.1 as f64))
            .unwrap()
    });

    let mut max = 0;
    dfs(0, 0, w, &mut vw, &mut max);

    println!("{}", max)
}
