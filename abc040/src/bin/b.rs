use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let mut min = std::i32::MAX;
    for i in 1..=n {
        let j = (n / i) as i32;
        min = min.min((i as i32 - j).abs() + n as i32 - (i as i32 * j));
    }
    println!("{}", min);
}
