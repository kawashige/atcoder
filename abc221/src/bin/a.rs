use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize
    }

    println!(
        "{}",
        if a == b {
            1
        } else {
            32_usize.pow((a - b) as u32)
        }
    );
}
