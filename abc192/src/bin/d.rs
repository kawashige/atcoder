use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        x: Chars,
        m: u64,
    }

    let x: Vec<u64> = x
        .into_iter()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .rev()
        .collect();
    let d = *x.iter().max().unwrap();

    if x.len() == 1 {
        println!("{}", if x[0] <= m { 1 } else { 0 });
        return;
    }

    let mut s = d as u128;
    let mut e = m as u128 + 1;
    while 1 < e - s {
        let mid = (e + s) / 2;
        let mut sum = 0_u128;
        let mut radix = 1_u128;
        for i in 0..x.len() {
            if (m as u128) < sum {
                break;
            }
            sum += x[i] as u128 * radix;
            radix = radix.saturating_mul(mid);
        }

        if sum <= m as u128 {
            s = mid;
        } else {
            e = mid;
        }
    }

    println!("{}", s - d as u128);
}
