use proconio::input;
use std::cmp::Ordering;

fn partition(data: &[usize]) -> Option<(Vec<usize>, usize, Vec<usize>)> {
    match data.len() {
        0 => None,
        _ => {
            let (pivot_slice, tail) = data.split_at(1);
            let pivot = pivot_slice[0];
            let (left, right) = tail.iter().fold((vec![], vec![]), |mut splits, next| {
                {
                    let (ref mut left, ref mut right) = &mut splits;
                    if next < &pivot {
                        left.push(*next);
                    } else {
                        right.push(*next);
                    }
                }
                splits
            });

            Some((left, pivot, right))
        }
    }
}

fn select(data: &[usize], k: usize) -> Option<usize> {
    let part = partition(data);

    match part {
        None => None,
        Some((left, pivot, right)) => {
            let pivot_idx = left.len();

            match pivot_idx.cmp(&k) {
                Ordering::Equal => Some(pivot),
                Ordering::Greater => select(&left, k),
                Ordering::Less => select(&right, k - (pivot_idx + 1)),
            }
        }
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [[usize; n]; n]
    }

    let mid = if k % 2 == 0 { k * k / 2 - 1 } else { k * k / 2 };
    let mut nums = Vec::with_capacity(n * n);
    for i in 0..n {
        for j in 0..n {
            nums.push((a[i][j], (i, j)));
        }
    }
    nums.sort_unstable();
    let mut map = vec![vec![0; n]; n];
    nums.into_iter()
        .enumerate()
        .for_each(|(i, (_, p))| map[p.0][p.1] = i);

    let mut v = vec![std::usize::MAX; n * n];

    let mut min = std::usize::MAX;
    for j in 0..=(n - k) {
        for l in 0..k {
            for m in j..(j + k) {
                v[map[l][m]] = a[l][m];
            }
        }
        min = std::cmp::min(min, select(&v, mid).unwrap());

        for i in 1..=(n - k) {
            for m in j..(j + k) {
                v[map[i - 1][m]] = std::usize::MAX;
                v[map[i + k - 1][m]] = a[i + k - 1][m];
            }
            min = std::cmp::min(min, select(&v, mid).unwrap());
        }
    }

    println!("{}", min)
}
