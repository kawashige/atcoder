use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    let m = 1_000_000_007;
    let mut remains = [0; 13];

    if s[0] == '?' {
        (0..10).for_each(|i| remains[i] = 1);
    } else {
        remains[s[0].to_digit(10).unwrap() as usize] = 1;
    }

    for i in 1..s.len() {
        let mut next = [0; 13];
        for j in 0..13 {
            if remains[j] == 0 {
                continue;
            }
            if s[i] == '?' {
                (0..10).for_each(|k| {
                    next[(j * 10 + k) % 13] += remains[j];
                    next[(j * 10 + k) % 13] %= m;
                });
            } else {
                next[(j * 10 + s[i].to_digit(10).unwrap() as usize) % 13] += remains[j];
                next[(j * 10 + s[i].to_digit(10).unwrap() as usize) % 13] %= m;
            }
        }
        remains = next;
    }

    println!("{}", remains[5]);
}
