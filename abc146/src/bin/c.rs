use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
        x: u64
    }

    if let Some(i) = (1_u64..11)
        .rev()
        .find(|i| 10_u64.pow(*i as u32 - 1) * a + b * i <= x)
    {
        let num = std::cmp::min(10_u64.pow(i as u32) - 1, (x - b * i) / a);
        println!("{}", std::cmp::min(1_000_000_000, num));
    } else {
        println!("0");
    }
}
