use proconio::input;

fn main() {
    input! {
        n: usize,
        mut xy: [(i32, i32); n]
    }

    let mut x = Vec::with_capacity(n);
    let mut y = Vec::with_capacity(n);
    for i in 0..n {
        x.push(xy[i].0);
        y.push(xy[i].1);
    }
    x.sort_unstable();
    y.sort_unstable();

    let p = (x[n / 2], y[n / 2]);
    let mut r: i64 = 0;
    for (x, y) in xy {
        r += (x - p.0).abs() as i64 + (y - p.1).abs() as i64;
    }

    println!("{}", r);
}
