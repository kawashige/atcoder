use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i32, i32); n]
    }

    let mut x = Vec::with_capacity(n);
    let mut y = Vec::with_capacity(n);

    for i in 0..n {
        x.push((xy[i].0, i));
        y.push((xy[i].1, i));
    }

    x.sort_unstable();
    y.sort_unstable();

    if (x[0].1, x[n - 1].1) == (y[0].1, y[n - 1].1) || (x[0].1, x[n - 1].1) == (y[n - 1].1, y[0].1)
    {
        println!(
            "{}",
            (x[n - 2].0 - x[0].0)
                .max(x[n - 1].0 - x[1].0)
                .max(y[n - 2].0 - y[0].0)
                .max(y[n - 1].0 - y[1].0)
        )
    } else if x[n - 1].0 - x[0].0 < y[n - 1].0 - y[0].0 {
        println!(
            "{}",
            (x[n - 1].0 - x[0].0)
                .max(y[n - 2].0 - y[0].0)
                .max(y[n - 1].0 - y[1].0)
        );
    } else {
        println!(
            "{}",
            (y[n - 1].0 - y[0].0)
                .max(x[n - 2].0 - x[0].0)
                .max(x[n - 1].0 - x[1].0)
        );
    }
}
