use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: u128,
        a: [u128; n],
        b: [u128; m],
    }

    let mut sum = 0;
    let mut i = 0;

    while i < n && sum + a[i] <= k {
        sum += a[i];
        i += 1;
    }

    let mut max = i;
    let mut j = 0;
    while j < m && sum <= k {
        sum += b[j];
        j += 1;
        while 0 < i && sum > k {
            sum -= a[i - 1];
            i -= 1;
        }
        if sum <= k {
            max = std::cmp::max(max, i + j);
        }
    }

    println!("{}", max);
}
