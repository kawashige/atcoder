use proconio::input;

fn main() {
    input! {
        k: usize
    }

    let mut nums = (1..=9).collect::<Vec<u64>>();

    let mut s = 0;
    while nums.len() < k {
        let e = nums.len();
        for i in s..e {
            let n = nums[i] * 10;
            let d = nums[i] % 10;
            match d {
                0 => {
                    nums.push(n);
                    nums.push(n + 1);
                }
                9 => {
                    nums.push(n + 8);
                    nums.push(n + 9);
                }
                _ => {
                    nums.push(n + d - 1);
                    nums.push(n + d);
                    nums.push(n + d + 1);
                }
            }
            if nums.len() >= k {
                break;
            }
        }
        s = e;
    }
    println!("{}", nums[k - 1]);
}
