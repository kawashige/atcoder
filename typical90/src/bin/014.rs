use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
        mut b: [i32; n]
    }

    a.sort_unstable();
    b.sort_unstable();

    let count = (0..n).map(|i| (a[i] - b[i]).abs() as i64).sum::<i64>();

    println!("{}", count);
}
