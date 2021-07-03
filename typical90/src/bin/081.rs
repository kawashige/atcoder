use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(usize, usize); n]
    }

    let mut matrix = vec![vec![0; 5001]; 5001];

    for (a, b) in ab {
        matrix[a][b] += 1;
    }

    for i in 0..5001 {
        for j in 0..5001 {
            if i > 0 {
                matrix[i][j] += matrix[i - 1][j];
            }
            if j > 0 {
                matrix[i][j] += matrix[i][j - 1];
            }
            if i > 0 && j > 0 {
                matrix[i][j] -= matrix[i - 1][j - 1];
            }
        }
    }

    let mut r = 0;

    for i in k..5001 {
        for j in k..5001 {
            let mut sum = matrix[i][j];
            if i > k {
                sum -= matrix[i - k - 1][j];
            }
            if j > k {
                sum -= matrix[i][j - k - 1];
            }
            if i > k && j > k {
                sum += matrix[i - k - 1][j - k - 1];
            }
            r = std::cmp::max(r, sum);
        }
    }

    println!("{}", r);
}
