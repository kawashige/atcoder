use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n]
    }

    let mut sums: Vec<i128> = vec![0; n];
    let mut maxs: Vec<i128> = vec![0; n];
    let mut max = a[0] as i128;
    sums[0] = a[0] as i128;
    maxs[0] = a[0] as i128;
    for i in 1..n {
        sums[i] = sums[i - 1] + a[i] as i128;
        max = std::cmp::max(max, sums[i]);
        maxs[i] = max;
    }

    let mut result = 0;
    let mut sum = 0;
    for i in 0..n {
        result = std::cmp::max(result, sum + maxs[i]);
        sum += sums[i];
        result = std::cmp::max(result, sum);
    }

    println!("{}", result);
}
