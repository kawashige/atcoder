use proconio::input;

fn dfs(nums: &mut Vec<usize>, l: &Vec<i32>, a: i32, b: i32, c: i32, min: &mut i32) {
    if nums.len() == l.len() {
        let mut cost = 0;
        let mut sums = vec![0; 4];
        for i in 0..nums.len() {
            if nums[i] == 0 {
                continue;
            }
            if sums[nums[i]] != 0 {
                cost += 10;
            }
            sums[nums[i]] += l[i];
        }
        if (1..4).any(|i| sums[i] == 0) {
            return;
        }
        cost += (a - sums[1]).abs() + (b - sums[2]).abs() + (c - sums[3]).abs();
        *min = std::cmp::min(*min, cost);
        return;
    }

    for i in 0..4 {
        nums.push(i);
        dfs(nums, l, a, b, c, min);
        nums.pop();
    }
}

fn main() {
    input! {
        n: usize,
        a: i32,
        b: i32,
        c: i32,
        l: [i32; n]
    }

    let mut min = std::i32::MAX;
    dfs(&mut Vec::new(), &l, a, b, c, &mut min);
    println!("{}", min);
}
