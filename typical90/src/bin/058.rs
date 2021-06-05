use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u64
    }

    let m = 100_000;
    let mut nums = Vec::with_capacity(m);
    nums.push(n);
    let mut seen = vec![false; m];
    seen[n] = true;
    let mut x = n;
    for i in 1..=k {
        x = (x + x
            .to_string()
            .chars()
            .map(|v| v.to_digit(10).unwrap() as usize)
            .sum::<usize>())
            % m;

        if seen[x] {
            let j = (0..nums.len()).find(|j| nums[*j] == x).unwrap();
            let mut k = k;
            k -= i;
            k %= i - j as u64;
            println!("{}", nums[j + k as usize]);
            return;
        } else {
            seen[x] = true;
            nums.push(x);
        }
    }

    println!("{}", x);
}
