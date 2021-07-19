use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        c: u64,
        a: [[u64; w]; h]
    }
    const MAX: u64 = 10_000_000_000_000_000;

    let mut dp1 = vec![vec![0; w]; h];
    for i in (0..h).rev() {
        for j in (0..w).rev() {
            dp1[i][j] = MAX;
            if i < h - 1 {
                dp1[i][j] = dp1[i][j].min(dp1[i + 1][j] + c).min(a[i + 1][j] + c);
            }
            if j < w - 1 {
                dp1[i][j] = dp1[i][j].min(dp1[i][j + 1] + c).min(a[i][j + 1] + c);
            }
        }
    }

    let mut dp2 = vec![vec![0; w]; h];
    for i in (0..h).rev() {
        for j in 0..w {
            dp2[i][j] = MAX;
            if i < h - 1 {
                dp2[i][j] = dp2[i][j].min(dp2[i + 1][j] + c).min(a[i + 1][j] + c);
            }
            if 0 < j {
                dp2[i][j] = dp2[i][j].min(dp2[i][j - 1] + c).min(a[i][j - 1] + c);
            }
        }
    }

    let mut r = std::u64::MAX;
    for i in 0..h {
        for j in 0..w {
            let cost = a[i][j] + dp1[i][j].min(dp2[i][j]);
            r = r.min(cost);
        }
    }

    println!("{}", r);
}
