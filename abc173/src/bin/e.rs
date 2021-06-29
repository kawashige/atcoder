use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [i64; n]
    }
    const M: i64 = 1_000_000_007;

    a.sort_unstable_by(|a, b| b.abs().cmp(&a.abs()));

    let min_count = (0..k).filter(|i| a[*i] < 0).count();

    if min_count % 2 == 1 {
        let k_m = (0..k).rev().find(|i| a[*i] < 0).unwrap();
        let k_min_plus_i = (0..k).rev().find(|i| a[*i] > 0);
        let r_max_plus_i = (k..n).find(|i| a[*i] > 0);
        let r_max_minus_i = (k..n).find(|i| a[*i] < 0);

        match (k_min_plus_i, r_max_plus_i, r_max_minus_i) {
            (Some(k_p), Some(r_p), Some(r_m)) => {
                if a[k_p] * a[r_p] > a[k_m] * a[r_m] {
                    a.swap(k_m, r_p);
                } else {
                    a.swap(k_p, r_m);
                }
            }
            (_, Some(r_p), _) => {
                a.swap(k_m, r_p);
            }
            (Some(k_p), None, Some(r_m)) => {
                a.swap(k_p, r_m);
            }
            _ => {
                if a.last().unwrap() == &0 {
                    println!("0");
                    return;
                }
                a = a.into_iter().rev().collect();
            }
        }
    }

    let mut r = (0..k).fold(1, |acc, i| acc * a[i] % M);
    while r < 0 {
        r += M;
    }
    println!("{}", r);
}
