use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [[usize; n]; n],
        q: usize,
        p: [usize; q]
    }

    let mut acc = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            acc[i][j] = d[i][j];
            if i > 0 {
                acc[i][j] += acc[i - 1][j];
            }
            if j > 0 {
                acc[i][j] += acc[i][j - 1];
            }
            if i > 0 && j > 0 {
                acc[i][j] -= acc[i - 1][j - 1];
            }
        }
    }

    let mut max = vec![0; n * n + 1];
    for i in 0..n {
        for j in 0..n {
            for k in 0..(n - i) {
                for l in 0..(n - j) {
                    let mut s = acc[i + k][j + l];
                    if i > 0 && j > 0 {
                        s += acc[i - 1][j - 1];
                    }
                    if i > 0 {
                        s -= acc[i - 1][j + l];
                    }
                    if j > 0 {
                        s -= acc[i + k][j - 1];
                    }

                    max[(k + 1) * (l + 1)] = max[(k + 1) * (l + 1)].max(s);
                }
            }
        }
    }

    let mut acc_max = vec![0; n * n + 1];
    for i in 0..acc_max.len() {
        acc_max[i] = if i > 0 {
            max[i].max(acc_max[i - 1])
        } else {
            max[i]
        }
    }

    for x in p {
        println!("{}", acc_max[x]);
    }
}
