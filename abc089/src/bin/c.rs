use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: [Chars; n]
    }

    let mut count = vec![0; 5];
    for name in s {
        match name[0] {
            'M' => count[0] += 1,
            'A' => count[1] += 1,
            'R' => count[2] += 1,
            'C' => count[3] += 1,
            'H' => count[4] += 1,
            _ => {}
        }
    }

    let mut sum: u64 = 0;
    for i in 0..5 {
        for j in (i + 1)..5 {
            for k in (j + 1)..5 {
                sum += count[i] * count[j] * count[k];
            }
        }
    }

    println!("{}", sum);
}
