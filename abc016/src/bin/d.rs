use proconio::input;

fn cross(a: (i64, i64), b: (i64, i64), c: (i64, i64), d: (i64, i64)) -> bool {
    let s = (b.0 - a.0) * (c.1 - a.1) - (c.0 - a.0) * (b.1 - a.1);
    let t = (b.0 - a.0) * (d.1 - a.1) - (d.0 - a.0) * (b.1 - a.1);

    let s2 = (d.0 - c.0) * (a.1 - c.1) - (a.0 - c.0) * (d.1 - c.1);
    let t2 = (d.0 - c.0) * (b.1 - c.1) - (b.0 - c.0) * (d.1 - c.1);
    s * t < 0 && s2 * t2 < 0
}

fn main() {
    input! {
        a: (i64, i64),
        b: (i64, i64),
        n: usize,
        xy: [(i64, i64); n]
    }

    let mut count = 0;
    for i in 0..n {
        if cross(a, b, xy[i], xy[(i + 1) % n]) {
            count += 1;
        }
    }

    println!("{}", 1 + count / 2);
}
