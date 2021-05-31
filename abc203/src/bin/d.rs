use proconio::input;
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [[usize; n]; n]
    }
    let num = ((k * k) / 2 + 1) as i32;

    let mut s = -1;
    let mut e = *(0..n).map(|i| a[i].iter().max().unwrap()).max().unwrap() as i32;

    while s + 1 < e {
        let mid = (s + e) / 2;

        let mut acc = vec![vec![0; n]; n];
        for i in 0..n {
            for j in 0..n {
                if a[i][j] as i32 > mid {
                    acc[i][j] += 1;
                }
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

        let mut found = false;
        for i in 0..(n - k + 1) {
            if found {
                break;
            }
            for j in 0..(n - k + 1) {
                let mut count = acc[i + k - 1][j + k - 1];
                if i > 0 && j > 0 {
                    count += acc[i - 1][j - 1];
                }
                if i > 0 {
                    count -= acc[i - 1][j + k - 1];
                }
                if j > 0 {
                    count -= acc[i + k - 1][j - 1];
                }
                if count < num {
                    found = true;
                    break;
                }
            }
        }

        if found {
            e = mid;
        } else {
            s = mid;
        }
    }

    println!("{}", e)
}
