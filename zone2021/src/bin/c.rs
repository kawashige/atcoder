use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        v: [[usize; 5]; n]
    }

    let mut ok = 1;
    let mut ng = v.iter().map(|vv| vv.iter().max().unwrap()).max().unwrap() + 1;

    while ok + 1 < ng {
        let mid = (ok + ng) / 2;
        let mut set = HashSet::new();

        for i in 0..n {
            let mut x = 0;
            for j in 0..5 {
                if v[i][j] >= mid {
                    x |= 1 << j;
                }
            }
            set.insert(x);
        }

        let mut found = false;
        let nums = set.into_iter().collect::<Vec<_>>();
        let target = 2_usize.pow(5) - 1;

        for i in 0..nums.len() {
            if found {
                break;
            }
            if nums[i] == target {
                found = true;
                break;
            }
            for j in (i + 1)..nums.len() {
                if found {
                    break;
                }
                if nums[i] | nums[j] == target {
                    found = true;
                    break;
                }
                for k in (j + 1)..nums.len() {
                    if nums[i] | nums[j] | nums[k] == 2_usize.pow(5) - 1 {
                        found = true;
                        break;
                    }
                }
            }
        }

        if found {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}
