use proconio::input;

fn main() {
    input! {
        n: usize,
        nums: [(usize, usize); n]
    }

    let sum: usize = nums.into_iter().map(|(a, b)| (a..=b).sum::<usize>()).sum();

    println!("{}", sum);
}
