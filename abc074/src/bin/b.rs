use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        x: [usize; n]
    }

    let sum = x
        .into_iter()
        .map(|i| if i < k - i { i } else { k - i })
        .sum::<usize>();
    println!("{}", sum * 2);
}
