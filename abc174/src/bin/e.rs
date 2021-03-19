use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n]
    }

    a.sort_unstable();

    let mut s = 0;
    let mut e = a[n - 1];
    while 1 < e - s {
        let mid = (e + s) / 2;

        let sum = a.iter().map(|i| ((i - 1) / mid) as u128).sum::<u128>();
        if sum <= k as u128 {
            e = mid;
        } else {
            s = mid;
        }
    }

    println!("{}", e as u128);
}
