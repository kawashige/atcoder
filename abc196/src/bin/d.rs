use proconio::input;
use std::collections::HashMap;

fn recurse(
    filled: &mut Vec<char>,
    h: usize,
    w: usize,
    a: usize,
    b: usize,
    memo: &mut HashMap<(String, usize, usize), u32>,
) -> u32 {
    let key = filled.iter().collect::<String>();
    if let Some(c) = memo.get(&(key.clone(), a, b)) {
        return *c;
    }

    if let Some(i) = (0..filled.len()).find(|i| filled[*i] == '0') {
        filled[i] = '1';
        let mut count = 0;
        if b > 0 {
            count += recurse(filled, h, w, a, b - 1, memo);
        }
        if a > 0 {
            if (i + 1) % w != 0 && filled[i + 1] == '0' {
                filled[i + 1] = '1';
                count += recurse(filled, h, w, a - 1, b, memo);
                filled[i + 1] = '0';
            }
            if i / w < h - 1 && filled[i + w] == '0' {
                filled[i + w] = '1';
                count += recurse(filled, h, w, a - 1, b, memo);
                filled[i + w] = '0';
            }
        }
        filled[i] = '0';
        memo.insert((key, a, b), count);
        count
    } else {
        1
    }
}

fn main() {
    input! {
        h: usize,
        w: usize,
        a: usize,
        b: usize
    }

    let mut filled = vec!['0'; h * w];
    let count = recurse(&mut filled, h, w, a, b, &mut HashMap::new());
    println!("{}", count);
}
