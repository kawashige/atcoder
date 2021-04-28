use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut x: [i64; m]
    }

    if m <= n {
        println!("0");
        return;
    }

    x.sort_unstable();

    let mut v = (1..m).map(|i| x[i] - x[i - 1]).collect::<Vec<i64>>();
    v.sort_unstable();
    let sum = v.into_iter().take(m - n).sum::<i64>();

    println!("{}", sum);
}
