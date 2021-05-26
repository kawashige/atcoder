use proconio::input;

fn main() {
    input! {
        n: usize
    }
    let m = 1_000_000_007;

    let mut combination = vec![0; n + 1];
    for i in 2..=n {
        combination[i] = combination[i - 1] + i - 1;
    }

    println!("combination: {:?}", combination);

    let mut nums = vec![0; n + 1];
    nums[n] = n;

    for i in (1..n).rev() {
        nums[i] = nums[i + 1];
        for j in 1..(1 + i) {
            if j + i > n {
                break;
            }
            nums[i] += combination[(n - j) / i + 1];
            nums[i] %= m;
        }
    }

    // println!("{:?}", nums);

    for i in 1..=n {
        println!("{}", nums[i]);
    }
}
