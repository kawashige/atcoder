use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let mut digits = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    digits.sort_unstable_by_key(|d| -(*d as i32));

    let mut max = 0;

    for i in 1..(2_usize.pow(digits.len() as u32) - 1) {
        let (d1, d2) = (0..digits.len()).fold((0, 0), |(d1, d2), j| {
            if i & 1 << j == 0 {
                (d1 * 10 + digits[j], d2)
            } else {
                (d1, d2 * 10 + digits[j])
            }
        });
        max = max.max(d1 * d2);
    }

    println!("{}", max);
}
