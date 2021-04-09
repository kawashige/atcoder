use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n + 1],
        b: [i64; n]
    }

    let sum = a.iter().sum::<i64>();

    for i in 0..n {
        let tmp = a[i];
        a[i] = std::cmp::max(0, a[i] - b[i]);
        a[i + 1] = std::cmp::max(0, a[i + 1] - (b[i] - tmp + a[i]));
    }

    println!("{}", sum - a.iter().sum::<i64>());
}
