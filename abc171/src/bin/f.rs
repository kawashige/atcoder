use std::collections::HashMap;

use proconio::input;
use proconio::marker::Chars;

const M: usize = 1_000_000_007;

fn dfs(i: usize, j: usize, l: usize, memo: &mut HashMap<(usize, usize), usize>) -> usize {
    if i == 0 {
        if j == 0 {
            1
        } else {
            0
        }
    } else if i < j {
        0
    } else {
        if let Some(x) = memo.get(&(i, j)) {
            *x
        } else {
            let mut r = if j == 0 {
                0
            } else {
                dfs(i - 1, j - 1, l, memo)
            };
            r += dfs(i - 1, j, l, memo) * if j == l { 26 } else { 25 } % M;
            r %= M;
            memo.insert((i, j), r);
            r
        }
    }
}

fn main() {
    input! {
        k: usize,
        s: Chars
    }

    let r = dfs(k + s.len(), s.len(), s.len(), &mut HashMap::new());

    println!("{}", r);
}
