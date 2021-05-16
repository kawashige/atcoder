use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        uvw: [(usize, usize, u64); n - 1]
    }
    let mut list = vec![vec![]; n];
    for (u, v, w) in uvw {
        list[u - 1].push((v - 1, w));
        list[v - 1].push((u - 1, w));
    }

    let mut seen = vec![false; n];
    let mut nums = Vec::with_capacity(n - 1);
    let mut queue = VecDeque::new();

    for (next, weight) in &list[0] {
        queue.push_back((*next, *weight))
    }

    while let Some((node, weight)) = queue.pop_back() {
        nums.push(weight);
        seen[node] = true;

        for (n, w) in &list[node] {
            if seen[*n] {
                continue;
            }
            queue.push_back((*n, w ^ weight));
        }
    }

    let m = 1_000_000_007;
    let mut count_zero = [0_u64; 64];
    let mut count_one = [0_u64; 64];

    let mut sum = 0;
    for i in (0..n).rev() {
        let mut mul = 1;
        for j in 0..64 {
            if nums[i] & 1 << j == 0 {
                sum = (sum + count_one[j] * mul % m) % m;
                count_zero[j] += 1;
            } else {
                sum = (sum + count_zero[j] * mul % m) % m;
                count_one[j] += 1;
            }
            mul = (mul * 2) % m;
        }
    }
    println!("{}", sum);
}
