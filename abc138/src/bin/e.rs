use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        t: Chars
    }

    let mut indexes_s = vec![vec![]; 26];
    for i in 0..s.len() {
        indexes_s[s[i] as usize - 0x61].push(i as i32);
    }

    let mut count = 0;
    let mut prev = -1;
    for i in 0..t.len() {
        let target = &indexes_s[t[i] as usize - 0x61];
        if target.is_empty() {
            println!("-1");
            return;
        }
        match target.binary_search(&prev) {
            Ok(i) => {
                if i + 1 >= target.len() {
                    count += 1;
                }
                prev = target[(i + 1) % target.len()];
            }
            Err(i) => {
                if i >= target.len() {
                    count += 1;
                }
                prev = target[i % target.len()];
            }
        }
    }

    println!("{}", count as u64 * s.len() as u64 + prev as u64 + 1);
}
