use proconio::input;

fn main() {
    input! {
        n: usize,
        v: [usize; n],
        c: [usize; n]
    }

    let max = (0..n)
        .map(|i| if v[i] > c[i] { v[i] - c[i] } else { 0 })
        .sum::<usize>();

    println!("{}", max);
}
