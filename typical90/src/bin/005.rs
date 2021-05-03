use proconio::input;

fn mul(m1: &Vec<Vec<usize>>, m2: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let m = 1_000_000_007;
    let mut r = vec![vec![0; m2[0].len()]; m1.len()];

    for i in 0..m1.len() {
        for j in 0..m2[0].len() {
            for k in 0..m1[0].len() {
                r[i][j] += m1[i][k] * m2[k][j];
                r[i][j] %= m;
            }
        }
    }

    r
}

fn pow(m: &Vec<Vec<usize>>, n: u64) -> Vec<Vec<usize>> {
    if n == 1 {
        m.clone()
    } else if n % 2 == 0 {
        let r = pow(m, n / 2);
        mul(&r, &r)
    } else {
        mul(&m, &pow(m, n - 1))
    }
}

fn main() {
    input! {
        n: u64,
        b: usize,
        k: usize,
        c: [usize; k]
    }

    let mut bb = vec![vec![0; b]; b];

    for i in 0..b {
        for j in 0..k {
            bb[i][(i * 10 + c[j]) % b] += 1;
        }
    }

    let r = pow(&bb, n);

    println!("{}", r[0][0]);
}
