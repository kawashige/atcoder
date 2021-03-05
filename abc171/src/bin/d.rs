use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
        replaces: [(i128, i128); q]
    }

    let mut nums = vec![0; 100_001];
    let mut sum: u128 = 0;
    for num in a {
        nums[num as usize] += 1;
        sum += num as u128;
    }

    for (b, c) in replaces {
        sum = (sum as i128 + nums[b as usize] * (c - b)) as u128;
        nums[c as usize] += nums[b as usize];
        nums[b as usize] = 0;
        println!("{}", sum);
    }
}
