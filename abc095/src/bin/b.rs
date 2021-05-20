use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        mut m: [usize; n]
    }

    m.sort_unstable();
    let min = m[0];
    let sum = m.into_iter().sum::<usize>();
    let count = n + (x - sum) / min;

    println!("{}", count);
}
