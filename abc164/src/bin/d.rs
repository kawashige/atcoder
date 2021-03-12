use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    let mut mods = [0; 2019];
    let mut result = 0_u64;

    for c in s {
        let i = c.to_digit(10).unwrap() as usize;
        let mut new_mods = [0; 2019];
        for j in 0..2019 {
            if mods[j] == 0 {
                continue;
            }
            new_mods[(j * 10 + i) % 2019] += mods[j];
        }
        new_mods[i] += 1;
        result += new_mods[0];
        mods = new_mods;
    }

    println!("{}", result);
}
