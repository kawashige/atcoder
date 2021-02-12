use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; 2],
    }

    let mut sums = vec![vec![0; n + 1]; 2];
    for i in 0..2 {
        for j in 0..n {
            sums[i][j + 1] = sums[i][j] + a[i][j];
        }
    }

    let max = (0..n)
        .map(|i| sums[0][i + 1] + sums[1][n] - sums[1][i])
        .max()
        .unwrap();
    println!("{}", max);
}
