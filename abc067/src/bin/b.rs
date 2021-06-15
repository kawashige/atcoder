use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut l: [usize; n]
    }

    l.sort_unstable_by(|a, b| b.cmp(&a));
    let sum = l.into_iter().take(k).sum::<usize>();

    println!("{}", sum);
}
