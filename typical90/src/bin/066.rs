use proconio::input;

fn main() {
    input! {
        n: usize,
        lr: [(usize, usize); n]
    }

    let mut nums = vec![0.0; 101];

    let mut e = 0.0;
    for (l, r) in lr {
        let count = (r - l + 1) as f64;
        for i in l..=r {
            for j in (i + 1)..nums.len() {
                e += nums[j] / count;
            }

            nums[i] += 1.0 / count;
        }
    }

    println!("{}", e);
}
