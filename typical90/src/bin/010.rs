use proconio::input;

fn main() {
    input! {
        n: usize,
        cp: [(usize, usize); n],
        q: usize,
        lr: [(usize, usize); q]
    }

    let mut sum = vec![vec![0; n + 1]; 2];
    for i in 0..n {
        sum[0][i + 1] = sum[0][i];
        sum[1][i + 1] = sum[1][i];
        if cp[i].0 == 1 {
            sum[0][i + 1] += cp[i].1;
        } else {
            sum[1][i + 1] += cp[i].1;
        }
    }

    for (l, r) in lr {
        let sum1 = sum[0][r] - sum[0][l - 1];
        let sum2 = sum[1][r] - sum[1][l - 1];
        println!("{} {}", sum1, sum2);
    }
}
