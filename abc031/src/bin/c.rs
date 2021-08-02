use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n]
    }

    let mut max = None;

    for i in 0..n {
        let mut takahashi = 0;
        let mut aoki = 0;
        if i > 0 {
            let mut even = a[i - 1];
            let mut odd = a[i];
            aoki = odd;
            takahashi = even;
            for j in (0..(i - 1)).rev() {
                if (i - j) % 2 == 0 {
                    odd += a[j];
                    if aoki <= even {
                        aoki = even;
                        takahashi = odd;
                    }
                } else {
                    even += a[j];
                    if aoki <= odd {
                        aoki = odd;
                        takahashi = even;
                    }
                }
            }
        }

        if i < n - 1 {
            let mut odd = a[i];
            let mut even = a[i + 1];
            if i == 0 || aoki < even {
                aoki = even;
                takahashi = odd;
            }
            for j in (i + 2)..n {
                if (j - i) % 2 == 0 {
                    odd += a[j];
                } else {
                    even += a[j];
                }

                if aoki < even {
                    takahashi = odd;
                    aoki = even;
                }
            }
        }
        if max.is_none() {
            max = Some(takahashi);
        } else {
            max = max.map(|x| x.max(takahashi));
        }
    }

    println!("{}", max.unwrap());
}
