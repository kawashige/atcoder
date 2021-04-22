use proconio::input;

fn main() {
    input! {
        v: [usize; 5]
    }

    let sum = v.iter().map(|n| 10 * ((n + 9) / 10)).sum::<usize>();
    let max = v
        .iter()
        .map(|n| {
            if n % 10 == 0 {
                0
            } else {
                10 * ((n + 9) / 10) - n
            }
        })
        .max()
        .unwrap();

    println!("{}", sum - max);
}
