use proconio::input;

fn main() {
    input! {
        n: usize,
        p: u64,
        q: u64,
        a: [u64; n]
    }

    let mut count: u64 = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            let tmp1 = a[i] * a[j] % p;
            for k in (j + 1)..n {
                let tmp2 = tmp1 * a[k] % p;
                for l in (k + 1)..n {
                    let tmp3 = tmp2 * a[l] % p;
                    for m in (l + 1)..n {
                        if tmp3 * a[m] % p == q {
                            count += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{}", count);
}
