use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        xyc: [(usize, usize, char); n]
    }

    let mut white = vec![vec![0; k]; k];
    let mut black = vec![vec![0; k]; k];
    for (x, y, c) in xyc {
        if c == 'W' {
            if (x % (2 * k) >= k && y % (2 * k) >= k) || (x % (2 * k) < k && y % (2 * k) < k) {
                white[x % k][y % k] += 1;
            } else {
                black[x % k][y % k] += 1;
            }
        } else {
            if (x % (2 * k) >= k && y % (2 * k) >= k) || (x % (2 * k) < k && y % (2 * k) < k) {
                black[x % k][y % k] += 1;
            } else {
                white[x % k][y % k] += 1;
            }
        }
    }

    let mut top_left_white = vec![vec![0; k]; k];
    let mut top_right_white = vec![vec![0; k]; k];
    let mut bottom_left_white = vec![vec![0; k]; k];
    let mut bottom_right_white = vec![vec![0; k]; k];
    let mut top_left_black = vec![vec![0; k]; k];
    let mut top_right_black = vec![vec![0; k]; k];
    let mut bottom_left_black = vec![vec![0; k]; k];
    let mut bottom_right_black = vec![vec![0; k]; k];

    for i in 0..k {
        for j in 0..k {
            bottom_left_white[i][j] = white[i][j];
            bottom_left_black[i][j] = black[i][j];
            if i > 0 {
                bottom_left_white[i][j] += bottom_left_white[i - 1][j];
                bottom_left_black[i][j] += bottom_left_black[i - 1][j];
            }
            if j > 0 {
                bottom_left_white[i][j] += bottom_left_white[i][j - 1];
                bottom_left_black[i][j] += bottom_left_black[i][j - 1];
            }
            if i > 0 && j > 0 {
                bottom_left_white[i][j] -= bottom_left_white[i - 1][j - 1];
                bottom_left_black[i][j] -= bottom_left_black[i - 1][j - 1];
            }
        }
    }
    for i in 0..k {
        for j in (0..k).rev() {
            bottom_right_white[i][j] = white[i][j];
            bottom_right_black[i][j] = black[i][j];
            if i > 0 {
                bottom_right_white[i][j] += bottom_right_white[i - 1][j];
                bottom_right_black[i][j] += bottom_right_black[i - 1][j];
            }
            if j < k - 1 {
                bottom_right_white[i][j] += bottom_right_white[i][j + 1];
                bottom_right_black[i][j] += bottom_right_black[i][j + 1];
            }
            if i > 0 && j < k - 1 {
                bottom_right_white[i][j] -= bottom_right_white[i - 1][j + 1];
                bottom_right_black[i][j] -= bottom_right_black[i - 1][j + 1];
            }
        }
    }
    for i in (0..k).rev() {
        for j in 0..k {
            top_left_white[i][j] = white[i][j];
            top_left_black[i][j] = black[i][j];
            if i < k - 1 {
                top_left_white[i][j] += top_left_white[i + 1][j];
                top_left_black[i][j] += top_left_black[i + 1][j];
            }
            if j > 0 {
                top_left_white[i][j] += top_left_white[i][j - 1];
                top_left_black[i][j] += top_left_black[i][j - 1];
            }
            if i < k - 1 && j > 0 {
                top_left_white[i][j] -= top_left_white[i + 1][j - 1];
                top_left_black[i][j] -= top_left_black[i + 1][j - 1];
            }
        }
    }
    for i in (0..k).rev() {
        for j in (0..k).rev() {
            top_right_white[i][j] = white[i][j];
            top_right_black[i][j] = black[i][j];
            if i < k - 1 {
                top_right_white[i][j] += top_right_white[i + 1][j];
                top_right_black[i][j] += top_right_black[i + 1][j];
            }
            if j < k - 1 {
                top_right_white[i][j] += top_right_white[i][j + 1];
                top_right_black[i][j] += top_right_black[i][j + 1];
            }
            if i < k - 1 && j < k - 1 {
                top_right_white[i][j] -= top_right_white[i + 1][j + 1];
                top_right_black[i][j] -= top_right_black[i + 1][j + 1];
            }
        }
    }

    let mut max = 0;
    for i in 0..k {
        for j in 0..k {
            let r1 = if i > 0 && j > 0 {
                bottom_left_white[i - 1][j - 1]
            } else {
                0
            } + if i > 0 {
                bottom_right_black[i - 1][j]
            } else {
                0
            } + if j > 0 { top_left_black[i][j - 1] } else { 0 }
                + top_right_white[i][j];
            max = std::cmp::max(max, r1);

            let r2 = if i > 0 && j > 0 {
                bottom_left_black[i - 1][j - 1]
            } else {
                0
            } + if i > 0 {
                bottom_right_white[i - 1][j]
            } else {
                0
            } + if j > 0 { top_left_white[i][j - 1] } else { 0 }
                + top_right_black[i][j];
            max = std::cmp::max(max, r2);
        }
    }

    println!("{}", max);
}
