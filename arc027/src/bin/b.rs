use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        mut s1: Chars,
        mut s2: Chars
    }

    let mut map = ['a'; 26];
    let mut changed = true;

    while changed {
        changed = false;
        for i in 0..n {
            if s1[i].is_ascii_digit() && !s2[i].is_ascii_digit() {
                map[s2[i] as usize - 0x41] = s1[i];
                s2[i] = s1[i];
                changed = true;
            } else if !s1[i].is_ascii_digit() && s2[i].is_ascii_digit() {
                map[s1[i] as usize - 0x41] = s2[i];
                s1[i] = s2[i];
                changed = true;
            } else if !s1[i].is_ascii_digit() && map[s1[i] as usize - 0x41].is_ascii_digit() {
                s1[i] = map[s1[i] as usize - 0x41];
                changed = true;
            } else if !s2[i].is_ascii_digit() && map[s2[i] as usize - 0x41].is_ascii_digit() {
                s2[i] = map[s2[i] as usize - 0x41];
                changed = true;
            }
        }
    }

    let mut r: u64 = 1;
    let mut seen = [false; 26];

    for i in 0..n {
        if s1[i].is_ascii_digit() {
            continue;
        }
        if !seen[s1[i] as usize - 0x41] && !seen[s2[i] as usize - 0x41] {
            if i == 0 {
                r *= 9
            } else {
                r *= 10;
            }
        }
        seen[s1[i] as usize - 0x41] = true;
        seen[s2[i] as usize - 0x41] = true;
    }

    println!("{}", r);
}
