use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i128; n]
    }

    let mut sum: i128 = 0;
    let mut square_sum: i128 = 0;
    for num in &a {
        sum += num;
        square_sum += num * num;
    }

    let result: i128 = (0..n)
        .map(|i| {
            square_sum -= a[i] * a[i];
            sum -= a[i];
            ((n - i - 1) as i128) * a[i] * a[i] + square_sum - 2 * a[i] * sum
        })
        .sum();

    println!("{}", result);
}
