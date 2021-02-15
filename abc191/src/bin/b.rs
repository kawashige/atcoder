use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        nums: [usize; n]
    }

    println!(
        "{}",
        nums.into_iter()
            .filter(|n| n != &x)
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    )
}
