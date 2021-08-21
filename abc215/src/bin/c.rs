use std::collections::HashSet;

use proconio::input;
use proconio::marker::Chars;

fn dfs(
    k: usize,
    seen: &mut Vec<bool>,
    s: &Vec<char>,
    str: &mut String,
    result: &mut HashSet<String>,
) {
    if str.len() == seen.len() {
        result.insert(str.clone());
        return;
    }

    for i in 0..seen.len() {
        if seen[i] {
            continue;
        }
        seen[i] = true;
        str.push(s[i]);

        dfs(k, seen, s, str, result);
        if result.len() == k {
            return;
        }
        seen[i] = false;
        str.pop();
    }
}

fn main() {
    input! {
        mut s: Chars,
        k: usize,
    }

    s.sort_unstable();
    let mut result = HashSet::new();

    dfs(
        k,
        &mut vec![false; s.len()],
        &s,
        &mut String::new(),
        &mut result,
    );

    let mut r = result.into_iter().collect::<Vec<_>>();
    r.sort_unstable();

    println!("{}", r.last().unwrap());
}
