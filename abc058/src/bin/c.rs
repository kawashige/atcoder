use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: [Chars; n]
    }

    let mut count = vec![50; 26];
    for str in s {
        let str_count = str.into_iter().fold(vec![0; 26], |mut count, c| {
            count[c as usize - 0x61] += 1;
            count
        });
        for i in 0..26 {
            count[i] = std::cmp::min(count[i], str_count[i]);
        }
    }

    let mut r = String::new();
    for i in 0..26 {
        for _ in 0..count[i] {
            r.push((0x61 + i as u8) as char);
        }
    }

    println!("{}", r);
}
