use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let max = *a.iter().max().unwrap();
    let result = (2..=max)
        .max_by_key(|i| a.iter().filter(|j| **j % *i == 0).count())
        .unwrap();
    println!("{}", result);
}
