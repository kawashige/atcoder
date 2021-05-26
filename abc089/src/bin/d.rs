use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        d: usize,
        a: [[usize; w]; h],
        q: usize,
        lr: [(usize, usize); q]
    }

    let mut nums = vec![(0, 0); h * w + 1];
    for i in 0..h {
        for j in 0..w {
            nums[a[i][j]] = (i, j);
        }
    }

    let mut ds = vec![vec![]; d];
    let mut j = if d == 1 { 0 } else { 1 };
    for i in 1..nums.len() {
        ds[j].push(nums[i]);
        j += 1;
        j %= d;
    }

    let mut acc = vec![vec![]; d];
    for i in 0..d {
        acc[i] = vec![0; ds[i].len()];
        for j in 1..ds[i].len() {
            acc[i][j] = acc[i][j - 1]
                + (ds[i][j].0 as i32 - ds[i][j - 1].0 as i32).abs() as usize
                + (ds[i][j].1 as i32 - ds[i][j - 1].1 as i32).abs() as usize;
        }
    }

    for (l, r) in lr {
        let i = l % d;
        let cost = if i == 0 {
            acc[i][r / d - 1] - acc[i][l / d - 1]
        } else {
            acc[i][r / d] - acc[i][l / d]
        };
        println!("{}", cost);
    }
}
