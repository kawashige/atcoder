use proconio::input;

fn main() {
    input! {
        n: usize
    }
    let mut nums = (1..7).collect::<Vec<_>>();

    let x = n % 30;

    for i in 0..x {
        nums.swap(i % 5, i % 5 + 1);
    }

    println!(
        "{}",
        nums.into_iter().map(|d| d.to_string()).collect::<String>()
    );
}
