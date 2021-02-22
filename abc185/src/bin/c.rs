use proconio::input;

fn main() {
    input! {
        l: u128
    }

    println!(
        "{}",
        ((l - 11)..l).product::<u128>() / (1..12).product::<u128>()
    );
}
