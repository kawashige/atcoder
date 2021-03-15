use proconio::input;

fn main() {
    input! {
        k: usize,
        n: usize,
        a: [usize; n]
    }

    let max = (0..n)
        .map(|i| {
            if i == 0 {
                a[i] + k - a[n - 1]
            } else {
                a[i] - a[i - 1]
            }
        })
        .max()
        .unwrap();
    println!("{}", k - max);
}
