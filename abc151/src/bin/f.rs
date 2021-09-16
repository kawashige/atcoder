use proconio::input;

fn intersection(r: f64, a: (f64, f64), b: (f64, f64)) -> Vec<(f64, f64)> {
    let c = ((a.0 + b.0) / 2.0, (a.1 + b.1) / 2.0);
    let dist = calc_dist(a, b);
    let h = (r * r - (dist / 2.0) * (dist / 2.0)).sqrt();
    let aa = ((a.0 - b.0) / dist, (a.1 - b.1) / dist);
    let hp = (-aa.1 * h, aa.0 * h);
    let hm = (aa.1 * h, -aa.0 * h);

    vec![(c.0 + hp.0, c.1 + hp.1), (c.0 + hm.0, c.1 + hm.1)]
}

fn calc_dist(a: (f64, f64), b: (f64, f64)) -> f64 {
    ((a.0 - b.0).powf(2.0) + (a.1 - b.1).powf(2.0)).sqrt()
}

fn check(r: f64, xy: &Vec<(f64, f64)>) -> bool {
    let n = xy.len();
    for i in 0..n {
        for j in (i + 1)..n {
            if calc_dist(xy[i], xy[j]) / 2.0 > r {
                continue;
            }

            for p in intersection(r, xy[i], xy[j]) {
                if (0..n).all(|k| k == i || k == j || calc_dist(xy[k], p) <= r) {
                    return true;
                }
            }
        }
    }
    false
}

fn main() {
    input! {
        n: usize,
        xy: [(f64, f64); n]
    }

    let mut ok = 1000.0;
    let mut ng = 0.0;

    while ok - ng > 0.0000001 {
        let mid = (ok + ng) / 2.0;
        if check(mid, &xy) {
            ok = mid;
        } else {
            ng = mid
        }
    }

    println!("{}", ok);
}
