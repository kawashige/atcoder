use proconio::input;

fn dfs(
    nums: &mut Vec<usize>,
    n: usize,
    m: usize,
    q: &Vec<(usize, usize, usize, usize)>,
    max: &mut u128,
) {
    if nums.len() == n {
        let p = q
            .iter()
            .filter(|(a, b, c, _)| nums[*b - 1] - nums[*a - 1] == *c)
            .map(|(_, _, _, d)| *d as u128)
            .sum();
        *max = std::cmp::max(*max, p);
        return;
    }

    for i in *nums.last().unwrap_or(&1)..=m {
        nums.push(i);
        dfs(nums, n, m, q, max);
        nums.pop();
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        numq: [(usize, usize, usize, usize); q]
    }

    let mut max = 0;
    dfs(&mut Vec::new(), n, m, &numq, &mut max);

    println!("{}", max);
}
