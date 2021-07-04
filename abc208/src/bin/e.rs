use proconio::input;
use std::collections::HashMap;

fn dfs(i: usize, n: usize, k: usize, memo: &mut HashMap<(usize, usize, usize), u64>) -> u64 {
    if i == n {
        return if k < 10 { k as u64 } else { 9 } + if n == 1 { 0 } else { 1 };
    }

    if let Some(x) = memo.get(&(i, n, k)) {
        return *x;
    };

    let mut sum = 0;

    for j in 1..10 {
        sum += dfs(i + 1, n, k / j, memo);
    }

    if i != 1 {
        sum += 10_u64.pow((n - i) as u32);
    }

    memo.insert((i, n, k), sum);

    sum
}

fn main() {
    input! {
        n: u64,
        k: usize
    }

    let n = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();

    let mut r = 0;
    let mut memo = HashMap::new();
    for i in 1..n.len() {
        r += dfs(1, i, k, &mut memo);
    }

    let mut k = k;
    let mut finish = false;
    for i in 0..n.len() {
        if finish {
            break;
        }
        for j in 0..=n[i] {
            if i == 0 && j == 0 {
                continue;
            }
            if i == n.len() - 1 {
                if k >= j {
                    r += 1;
                }
            } else {
                if j == n[i] {
                    if i == n.len() - 1 {
                        if k > j {
                            r += j as u64 + if n.len() == 1 { 0 } else { 1 };
                        } else {
                            r += k as u64 + if n.len() == 1 { 0 } else { 1 };
                        }
                    } else if j == 0 {
                        r += n[(i + 1)..].iter().fold(1, |acc, x| acc * (*x as u64 + 1));
                        finish = true;
                        break;
                    } else {
                        k /= j;
                    }
                } else if j == 0 {
                    r += 10_u64.pow((n.len() - i) as u32);
                } else {
                    r += dfs(i + 1, n.len(), k / j, &mut memo);
                }
            }
        }
    }

    println!("{}", r);
}
