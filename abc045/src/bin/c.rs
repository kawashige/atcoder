use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        chars: Chars
    }

    let l = chars.len();
    let mut sum = 0;
    for i in 0..(2_i32.pow(l as u32 - 1)) {
        let mut prev = 0;
        for j in 0..(l - 1) {
            if i & 1 << j != 0 {
                sum += &chars[prev..(j + 1)]
                    .iter()
                    .clone()
                    .collect::<String>()
                    .parse::<i64>()
                    .unwrap();
                prev = j + 1;
            }
        }
        sum += &chars[prev..]
            .iter()
            .clone()
            .collect::<String>()
            .parse::<i64>()
            .unwrap();
    }
    println!("{}", sum);
}
