use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _: usize,
        s: Chars
    }

    let mut count = [0; 3];
    let mut sum: u64 = 0;
    for i in (0..s.len()).rev() {
        sum += match s[i] {
            'R' => count[1] * count[2],
            'G' => count[0] * count[2],
            'B' => count[0] * count[1],
            _ => unreachable!(),
        };
        count[match s[i] {
            'R' => 0,
            'G' => 1,
            'B' => 2,
            _ => unreachable!(),
        }] += 1;
    }

    for i in 1..=(s.len() / 2) {
        for j in 0..(s.len() - 2 * i) {
            if s[j] != s[j + i] && s[j + i] != s[j + 2 * i] && s[j] != s[j + 2 * i] {
                sum -= 1;
            }
        }
    }

    println!("{}", sum);
}
