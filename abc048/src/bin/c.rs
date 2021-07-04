use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n]
    }

    let mut prev = a[0];
    let mut count = 0;
    for i in 1..n {
        if prev + a[i] > x {
            let d = prev + a[i] - x;
            count += d;
            prev = if a[i] < d { 0 } else { a[i] - d };
        } else {
            prev = a[i];
        }
    }

    println!("{}", count);
}
