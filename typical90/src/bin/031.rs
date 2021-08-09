use std::collections::HashMap;

use proconio::input;

fn recurse(w: usize, b: usize, memo: &mut HashMap<(usize, usize), bool>) -> bool {
    if w == 0 && b < 2 {
        return false;
    }

    if let Some(b) = memo.get(&(w, b)) {
        return *b;
    }

    let mut result = true;
    if w > 0 {
        result &= recurse(w - 1, b + w, memo);
    }
    for i in 1..=(b / 2) {
        result &= recurse(w, b - i, memo);
    }

    memo.insert((w, b), !result);
    !result
}

fn main() {
    input! {
        n: usize,
        w: [usize; n],
        b: [usize; n],
    }

    let mut memo = HashMap::new();
    let mut count = 0;
    for i in 0..n {
        if recurse(w[i], b[i], &mut memo) {
            count += 1;
        } else {
            count += 2;
        }
    }

    println!("{}", if count % 2 == 0 { "Second" } else { "First" });
}
