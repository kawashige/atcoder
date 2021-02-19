use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize
    }

    let a = a
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum::<u32>();
    let b = b
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum::<u32>();
    if a < b {
        println!("{}", b);
    } else {
        println!("{}", a);
    }
}
