use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    if s.len() < 3 {}

    let mut counts = s.iter().fold([0; 10], |mut counts, c| {
        counts[*c as usize - 0x30] += 1;
        counts
    });

    if s.len() < 3 {
        counts[0] += 3 - s.len();
    }

    for i in 1..10 {
        if counts[i] == 0 {
            continue;
        }
        counts[i] -= 1;
        for j in if s.len() < 2 { 0..1 } else { 1..10 } {
            if counts[j] == 0 {
                continue;
            }
            counts[j] -= 1;
            for k in if s.len() < 3 { 0..1 } else { 1..10 } {
                if counts[k] == 0 {
                    continue;
                }
                if (i + 2 * j + 4 * k) % 8 == 0 {
                    println!("Yes");
                    return;
                }
            }
            counts[j] += 1;
        }
        counts[i] += 1;
    }
    println!("No");
}
