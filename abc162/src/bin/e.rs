use proconio::input;

fn mod_pow(x: i64, n: i64) -> i64 {
    const M: i64 = 1_000_000_007;
    if n == 0 {
        1
    } else if n % 2 == 0 {
        let r = mod_pow(x, n / 2);
        r * r % M
    } else {
        x * mod_pow(x, n - 1) % M
    }
}

fn main() {
    input! {
        n: usize,
        k: usize
    }

    const M: i64 = 1_000_000_007;

    let mut nums = vec![0; k + 1];

    let mut r = 0;
    for i in (1..=k).rev() {
        let c = k / i;
        let mut sum = mod_pow(c as i64, n as i64);
        for j in ((i + i)..=k).step_by(i) {
            sum -= nums[j];
            if sum < 0 {
                sum += M;
            }
        }
        nums[i] = sum;
        r += sum * i as i64 % M;
        r %= M;
    }

    println!("{}", r);
}
