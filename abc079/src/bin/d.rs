use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [[i32; 10]; 10],
        a: [[i32; w]; h]
    }

    let mut d = vec![vec![0; 10]; 10];
    for i in 0..10 {
        for j in 0..10 {
            if i == j {
                d[i][j] = 0;
            } else {
                d[i][j] = c[i][j];
            }
        }
    }

    for k in 0..10 {
        for i in 0..10 {
            for j in 0..10 {
                if d[i][j] > d[i][k] + d[k][j] {
                    d[i][j] = d[i][k] + d[k][j];
                }
            }
        }
    }

    let mut result = 0;
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == 1 || a[i][j] == -1 {
                continue;
            }
            result += d[a[i][j] as usize][1];
        }
    }

    println!("{}", result);
}
