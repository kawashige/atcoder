use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n]
    }

    let max = a.iter().sum::<usize>();

    let mut divides = BTreeSet::new();
    for i in 1..=((max as f64).sqrt() as usize) {
        if max % i == 0 {
            divides.insert(i);
            divides.insert(max / i);
        }
    }

    for i in divides.into_iter().rev() {
        let mut nums = a.iter().map(|x| x % i).collect::<Vec<usize>>();
        nums.sort_unstable();

        let mut c = 0;
        for j in 0..n {
            if nums[j] == 0 {
                continue;
            }

            let mut r = nums[j];
            c += r;
            for k in ((j + 1)..n).rev() {
                if nums[k] % i != 0 {
                    if i - nums[k] % i < r {
                        r -= i - nums[k] % i;
                        nums[k] = 0;
                    } else {
                        nums[k] = (nums[k] + r) % i;
                        break;
                    }
                }
            }
        }

        if c <= k {
            println!("{}", i);
            return;
        }
    }
}
