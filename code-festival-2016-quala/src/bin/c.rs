use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars,
        mut k: usize
    }

    for i in 0..s.len() {
        if s[i] == 'a' {
            continue;
        }
        let x = 26 - (s[i] as usize - 0x61);
        if k >= x {
            k -= x;
            s[i] = 'a';
        }
    }

    if k > 0 {
        let n = s.len() - 1;
        s[n] = (((s[n] as usize - 0x61 + k) % 26 + 0x61) as u8) as char;
    }

    println!("{}", s.into_iter().collect::<String>());
}
