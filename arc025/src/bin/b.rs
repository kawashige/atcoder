use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [[usize; w]; h]
    }

    let mut white = vec![vec![0; w]; h];
    let mut black = vec![vec![0; w]; h];

    for i in 0..h {
        for j in 0..w {
            if (i + j) % 2 == 0 {
                black[i][j] = c[i][j];
            } else {
                white[i][j] = c[i][j];
            }
            if i > 0 {
                black[i][j] += black[i - 1][j];
                white[i][j] += white[i - 1][j];
            }
            if j > 0 {
                black[i][j] += black[i][j - 1];
                white[i][j] += white[i][j - 1];
            }
            if i > 0 && j > 0 {
                black[i][j] -= black[i - 1][j - 1];
                white[i][j] -= white[i - 1][j - 1];
            }
        }
    }

    let mut r = 0;
    for i1 in 0..h {
        for i2 in i1..h {
            for j1 in 0..w {
                for j2 in j1..w {
                    let mut b = black[i2][j2];
                    let mut w = white[i2][j2];
                    if i1 > 0 && j1 > 0 {
                        b += black[i1 - 1][j1 - 1];
                        w += white[i1 - 1][j1 - 1];
                    }
                    if i1 > 0 {
                        b -= black[i1 - 1][j2];
                        w -= white[i1 - 1][j2];
                    }
                    if j1 > 0 {
                        b -= black[i2][j1 - 1];
                        w -= white[i2][j1 - 1];
                    }

                    if b == w {
                        r = r.max((i2 - i1 + 1) * (j2 - j1 + 1));
                    }
                }
            }
        }
    }

    println!("{}", r);
}
