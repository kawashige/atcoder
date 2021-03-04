use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let sum: u128 = (1..=n)
        .map(|i| {
            let count = n / i;
            let max = i * count;
            (count * (i + max) / 2) as u128
        })
        .sum();

    println!("{}", sum);
}
