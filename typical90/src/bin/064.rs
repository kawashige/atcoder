use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        lrv: [(usize, usize, i64); q]
    }

    let mut result = 0;
    for i in 0..(n - 1) {
        result += (a[i + 1] - a[i]).abs();
    }

    let mut d = vec![0; n];
    for (l, r, v) in lrv {
        let l = l - 1;
        let r = r - 1;

        if r < n - 1 {
            result += (a[r] + d[r] + v - a[r + 1]).abs() - (a[r] + d[r] - a[r + 1]).abs();
        }
        if l > 0 {
            result += (a[l] - a[l - 1] - d[l - 1] + v).abs() - (a[l] - a[l - 1] - d[l - 1]).abs();
        }

        if l > 0 {
            d[l - 1] -= v;
        }
        if r < n - 1 {
            d[r] += v;
        }

        println!("{}", result);
    }
}
