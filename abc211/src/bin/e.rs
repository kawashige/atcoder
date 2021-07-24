use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: [Chars; n]
    }
    let mut r: u64 = 0;

    let mut top = vec![vec![vec![0; k + 1]; n]; n];
    for i in 0..n {
        for j in (0..n).rev() {
            if s[i][j] == '#' {
                continue;
            }

            if i > 0 && s[i - 1][j] == '.' {
                top[i][j][0] = 1;
            }
            if j < n - 1 && s[i][j + 1] == '.' {
                top[i][j][1] += 1;
            }
            for k in 1..k {
                if i > 0 {
                    top[i][j][k + 1] += top[i - 1][j][k];
                }
                if j < n - 1 {
                    top[i][j][k + 1] += top[i][j + 1][k];
                }
            }
        }
    }

    let mut right = vec![vec![vec![0; k + 1]; n]; n];
    for i in (0..n).rev() {
        for j in (0..n).rev() {
            if s[i][j] == '#' {
                continue;
            }

            if i < n - 1 && s[i + 1][j] == '.' {
                right[i][j][0] = 1;
            }
            if j < n - 1 && s[i][j + 1] == '.' {
                right[i][j][1] += 1;
            }
            for k in 1..k {
                if i < n - 1 {
                    right[i][j][k + 1] += right[i + 1][j][k];
                }
                if j < n - 1 {
                    right[i][j][k + 1] += right[i][j + 1][k];
                }
            }
        }
    }

    let mut bottom = vec![vec![vec![0; k + 1]; n]; n];
    for i in (0..n).rev() {
        for j in 0..n {
            if s[i][j] == '#' {
                continue;
            }

            if i < n - 1 && s[i + 1][j] == '.' {
                bottom[i][j][0] = 1;
            }
            if j > 0 && s[i][j - 1] == '.' {
                bottom[i][j][1] += 1;
            }
            for k in 1..k {
                if i < n - 1 {
                    bottom[i][j][k + 1] += bottom[i + 1][j][k];
                }
                if j > 0 {
                    bottom[i][j][k + 1] += bottom[i][j - 1][k];
                }
            }
        }
    }

    let mut left = vec![vec![vec![0; k + 1]; n]; n];
    for j in 0..n {
        for i in 0..n {
            if s[i][j] == '#' {
                continue;
            }

            if i > 0 && s[i - 1][j] == '.' {
                left[i][j][0] = 1;
            }
            if j > 0 && s[i][j - 1] == '.' {
                left[i][j][1] += 1;
            }
            for k in 1..k {
                if j > 0 {
                    left[i][j][k + 1] += left[i][j - 1][k];
                }
                if i > 0 {
                    left[i][j][k + 1] += left[i - 1][j][k];
                }
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            if s[i][j] == '#' {
                continue;
            }
            let target = k - 1;
            for a in 0..=k {
                let mut v = top[i][j][a];
                if v == 0 {
                    continue;
                }
                for b in 0..k {
                    if a + b > target {
                        break;
                    }
                    v *= right[i][j + 1][b];
                    for c in 0..k {
                        if a + b + c > target {
                            break;
                        }
                        if i < n - 1 && c > 0 && bottom[i + 1][j][c] == 0 {
                            continue;
                        }
                        v *= if i == n - 1 || c == 0 {
                            1
                        } else {
                            bottom[i + 1][j][c]
                        };
                        for d in 0..k {
                            if a + b + c + d > target {
                                break;
                            }
                            if j > 0 && d > 0 && left[i][j - 1][d] == 0 {
                                continue;
                            }
                            if a + b + c + d == target {
                                println!(
                                    "i: {}, j: {}, a: {}, b: {}, c: {}, d: {}, {}, ",
                                    i, j, a, b, c, d, v,
                                );
                                r += v * if j == 0 || d == 0 {
                                    1
                                } else {
                                    left[i][j - 1][d]
                                };
                            }
                        }
                    }
                }
            }
            println!("i: {}, j: {}, r: {}", i, j, r);
        }
    }

    for i in 0..n {
        for j in 0..n {
            println!("i: {}, j: {}", i, j);
            println!("top: {:?}", top[i][j]);
            // println!("right: {:?}", right[i][j]);
            // println!("bottom: {:?}", bottom[i][j]);
            // println!("left: {:?}", left[i][j]);
        }
    }
    println!("r: {}", r);

    println!("{}", r / k as u64);
}
