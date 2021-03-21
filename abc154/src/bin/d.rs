use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [usize; n]
    }

    let mut e = vec![0.0; 1001];
    let mut sum = 0;
    for i in 1..=1000 {
        sum += i;
        e[i] = sum as f64 / i as f64;
    }

    let mut sum = (0..k).map(|i| e[p[i]]).sum::<f64>();
    let mut max = sum;
    for i in 1..(n - k + 1) {
        sum += e[p[i + k - 1]] - e[p[i - 1]];
        if max < sum {
            max = sum;
        }
    }

    println!("{}", max);
}
