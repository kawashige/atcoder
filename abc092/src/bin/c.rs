use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n]
    }

    let a = std::iter::once(0)
        .chain(a.into_iter())
        .chain(std::iter::once(0))
        .collect::<Vec<i32>>();

    let cost = (1..a.len()).map(|i| (a[i] - a[i - 1]).abs()).sum::<i32>();

    for i in 1..(a.len() - 1) {
        let diff = (a[i + 1] - a[i - 1]).abs() - (a[i] - a[i - 1]).abs() - (a[i + 1] - a[i]).abs();

        println!("{}", cost + diff);
    }
}
