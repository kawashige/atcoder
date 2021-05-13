use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i32; n]
    }

    let mut b = a
        .iter()
        .enumerate()
        .map(|(i, x)| x - i as i32 - 1)
        .collect::<Vec<i32>>();
    b.sort_unstable();
    let bb = b[n / 2];

    let r = a
        .into_iter()
        .enumerate()
        .map(|(i, x)| (x - i as i32 - 1 - bb).abs() as u64)
        .sum::<u64>();
    println!("{}", r);
}
