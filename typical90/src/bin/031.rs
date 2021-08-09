use proconio::input;

fn main() {
    input! {
        n: usize,
        w: [usize; n],
        b: [usize; n],
    }

    let mut grundy = vec![vec![0; 1555]; 55];

    for i in 0..51 {
        for j in 0..1501 {
            let mut mex = vec![0; 1555];
            if i >= 1 {
                mex[grundy[i - 1][j + i]] = 1;
            }
            if j >= 2 {
                for k in 1..=(j / 2) {
                    mex[grundy[i][j - k]] = 1;
                }
            }
            for k in 0..=1555 {
                if mex[k] == 0 {
                    grundy[i][j] = k;
                    break;
                }
            }
        }
    }

    let mut sum_xor = 0;
    for i in 0..n {
        sum_xor ^= grundy[w[i]][b[i]];
    }

    println!("{}", if sum_xor == 0 { "Second" } else { "First" });
}
