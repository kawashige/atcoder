use proconio::input;

fn main() {
    input! {
        n: String,
        k: usize
    }

    let mut x = n;
    for _ in 0..k {
        let mut num = x.chars().rev().enumerate().fold(0 as u64, |acc, (i, c)| {
            acc + 8_u64.pow(i as u32) as u64 * c.to_digit(10).unwrap() as u64
        });
        let mut chars = if num == 0 { vec!['0'] } else { vec![] };
        while num > 0 {
            let c = (b'0' + (num % 9) as u8) as char;
            chars.push(if c == '8' { '5' } else { c });
            num /= 9;
        }
        x = chars.into_iter().rev().collect::<String>();
    }

    println!("{}", x);
}
