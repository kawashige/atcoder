use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize
    }

    let mut sum = 0.0;
    for i in 1..=n {
        let mut j = i;
        let mut p = 1.0 / n as f64;
        while j < k {
            j *= 2;
            p *= 0.5;
        }
        sum += p;
    }

    println!("{}", sum);
}
