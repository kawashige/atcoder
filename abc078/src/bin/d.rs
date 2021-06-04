use proconio::input;
use std::collections::HashMap;

fn recurse(
    i: usize,
    j: usize,
    a: &Vec<i32>,
    v: i32,
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(x) = memo.get(&(i, j)) {
        return *x;
    }

    let mut result = (v - a.last().unwrap()).abs() as usize;
    for k in (i + 1)..a.len() {
        let r = recurse(k, (j + 1) % 2, a, a[k - 1], memo);
        result = if j == 0 {
            std::cmp::max(result, r)
        } else {
            std::cmp::min(result, r)
        };
    }

    memo.insert((i, j), result);
    result
}

fn main() {
    input! {
        n: usize,
        _z: i32,
        w: i32,
        a: [i32; n]
    }

    let r = recurse(0, 0, &a, w, &mut HashMap::new());

    println!("{}", r);
}
