use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut s_map = vec![None; 26];
    let mut t_map = vec![None; 26];
    for i in 0..s.len() {
        if s_map[s[i] as usize - 0x61].is_none() {
            s_map[s[i] as usize - 0x61] = Some(t[i]);
        }
        if t_map[t[i] as usize - 0x61].is_none() {
            t_map[t[i] as usize - 0x61] = Some(s[i]);
        }
        if t_map[t[i] as usize - 0x61] != Some(s[i]) || s_map[s[i] as usize - 0x61] != Some(t[i]) {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
