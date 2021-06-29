use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        mut a: [[i32; w]; h],
        b: [[i32; w]; h],
    }

    let mut count: u64 = 0;
    for i in 0..(h - 1) {
        for j in 0..(w - 1) {
            if a[i][j] < b[i][j] {
                let d = b[i][j] - a[i][j];
                a[i][j] += d;
                a[i + 1][j] += d;
                a[i][j + 1] += d;
                a[i + 1][j + 1] += d;
                count += d as u64;
            } else if a[i][j] > b[i][j] {
                let d = a[i][j] - b[i][j];
                a[i][j] -= d;
                a[i + 1][j] -= d;
                a[i][j + 1] -= d;
                a[i + 1][j + 1] -= d;
                count += d as u64;
            }
        }
    }

    if (0..h).all(|i| a[i][w - 1] == b[i][w - 1]) && (0..w).all(|j| a[h - 1][j] == b[h - 1][j]) {
        println!("Yes");
        println!("{}", count);
    } else {
        println!("No");
    }
}
