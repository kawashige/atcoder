use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        c: u64,
        a: [[u64; w]; h]
    }

    let mut rows = vec![std::u64::MAX; h];
    for i in 0..h {
        for j in 0..w {
            rows[i] = rows[i].min(a[i][j] + c * i as u64);
        }
    }
}
