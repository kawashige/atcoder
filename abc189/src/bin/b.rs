use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        drinks: [(usize, usize); n]
    }

    let mut sum = 0;
    for i in 0..n {
        sum += drinks[i].0 * drinks[i].1;
        if x * 100 < sum {
            println!("{}", i + 1);
            return;
        }
    }
    println!("-1");
}
