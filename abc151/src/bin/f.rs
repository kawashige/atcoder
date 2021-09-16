use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n]
    }

    let mut max = 0;
    let mut p = (0, 0);
    for i in 0..n {
        for j in 0..n {
            let d = (xy[i].0 - xy[j].0) * (xy[i].0 - xy[j].0)
                + (xy[i].1 - xy[j].1) * (xy[i].1 - xy[j].1);
            if d > max {
                max = d;
                p = (i, j);
            }
        }
    }

    let (gx, gy) = (
        (xy[p.0].0 + xy[p.1].0) as f64 / 2.0,
        (xy[p.0].1 + xy[p.1].1) as f64 / 2.0,
    );
    let mut max = std::f64::MIN;
    for i in 0..n {
        let d = ((gx - xy[i].0 as f64) * (gx - xy[i].0 as f64)
            + (gy - xy[i].1 as f64) * (gy - xy[i].1 as f64))
            .sqrt();
        if d > max {
            max = d;
        }
    }

    println!("{}", max);
}
