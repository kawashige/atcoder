use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: i64,
        mut a: [i64; n],
        mut f: [i64; n],
    }

    a.sort_unstable();
    f.sort_unstable_by(|a, b| b.cmp(&a));

    let max = (0..n).map(|i| a[i] * f[i]).max().unwrap();

    let mut s = -1;
    let mut e = max + 1;
    while s + 1 < e {
        let mid = (s + e) / 2;

        let mut possible = true;
        let mut remains = k;
        for i in 0..n {
            if a[i] * f[i] > mid {
                if remains < a[i] - mid / f[i] {
                    possible = false;
                    break;
                } else {
                    remains -= a[i] - mid / f[i];
                }
            }
        }

        if possible {
            e = mid;
        } else {
            s = mid
        }
    }

    println!("{}", e);
}
