use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n - 1],
    }

    let mut sum = 0;
    for i in 0..n {
        sum += b[a[i] - 1];
        if i > 0 && a[i - 1] + 1 == a[i] {
            sum += c[a[i - 1] - 1];
        }
    }

    println!("{}", sum);
}
