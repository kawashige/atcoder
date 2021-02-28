use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let max = (n as f64).sqrt() as usize;
    let mut nums = vec![false; max + 1];
    let mut count = 0;
    for i in 2..=max {
        if nums[i] {
            continue;
        }
        let mut num = i;
        while num * i <= n {
            num *= i;
            if num <= max {
                nums[num] = true;
            }
            count += 1;
        }
    }

    println!("{}", n - count);
}
