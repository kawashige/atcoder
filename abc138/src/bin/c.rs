use proconio::input;

fn main() {
    input! {
        n: usize,
        mut v: [usize; n]
    }

    v.sort_unstable();
    let mut sum = v[0];
    let mut mul = 1;
    for i in 1..n {
        sum += mul * v[i];
        mul *= 2;
    }

    println!("{}", sum as f64 / mul as f64);
}
