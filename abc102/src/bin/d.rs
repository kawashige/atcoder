use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }

    let sum = std::iter::once(0)
        .chain(a.into_iter().scan(0, |sum, x| {
            *sum += x;
            Some(*sum)
        }))
        .collect::<Vec<u64>>();

    let mut min = std::u64::MAX;

    for i in 2..(n - 1) {
        let i1 = sum[i] / 2;
        let sum1 = match sum.binary_search(&i1) {
            Ok(_) => i1,
            Err(j) => {
                if j < 2 {
                    sum[1]
                } else if j == i {
                    sum[j - 1]
                } else if i1 - sum[j - 1] < sum[j] - i1 {
                    sum[j - 1]
                } else {
                    sum[j]
                }
            }
        };

        let i2 = (sum[n] + sum[i]) / 2;
        let sum2 = match sum.binary_search(&i2) {
            Ok(_) => i2,
            Err(j) => {
                if j < i + 2 {
                    sum[i + 1]
                } else if j == n {
                    sum[n - 1]
                } else if i2 - sum[j - 1] < sum[j] - i2 {
                    sum[j - 1]
                } else {
                    sum[j]
                }
            }
        };

        let mut v = vec![sum1, sum[i] - sum1, sum2 - sum[i], sum[n] - sum2];
        v.sort_unstable();
        min = std::cmp::min(min, v[3] - v[0]);
    }

    println!("{}", min);
}
