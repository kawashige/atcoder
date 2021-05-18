use proconio::input;

fn main() {
    input! {
        x: usize
    }

    let mut max = 1;
    for i in 2..=((x as f64).sqrt() as usize) {
        let mut n = i;
        while n * i <= x {
            n *= i;
        }
        max = std::cmp::max(max, n);
    }

    println!("{}", max);
}
