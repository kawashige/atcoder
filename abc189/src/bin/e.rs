use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn main() {
    let n: usize = read();
    let p: Vec<Vec<i64>> = (0..n)
        .map(|_| {
            read::<String>()
                .split_ascii_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect();
    let m: usize = read();
    let ops: Vec<Vec<i64>> = (0..m)
        .map(|_| {
            read::<String>()
                .split_ascii_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect();
    let q: usize = read();
    let queries: Vec<Vec<usize>> = (0..q)
        .map(|_| {
            read::<String>()
                .split_ascii_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();

    let mut nums = vec![[0, 0]];
    let mut signs = vec![[1, 1]];
    let mut swaps = vec![false];

    for op in ops {
        let num = nums[nums.len() - 1];
        let sign = signs[signs.len() - 1];
        let swap = swaps[swaps.len() - 1];
        match op[0] {
            1 => {
                nums.push([num[1], -num[0]]);
                signs.push([sign[1], -sign[0]]);
                swaps.push(!swap);
            }
            2 => {
                nums.push([-num[1], num[0]]);
                signs.push([-sign[1], sign[0]]);
                swaps.push(!swap);
            }
            3 => {
                nums.push([2 * op[1] - num[0], num[1]]);
                signs.push([-sign[0], sign[1]]);
                swaps.push(swap);
            }
            4 => {
                nums.push([num[0], 2 * op[1] - num[1]]);
                signs.push([sign[0], -sign[1]]);
                swaps.push(swap);
            }
            _ => unreachable!(),
        }
    }

    for query in queries {
        let num = nums[query[0]];
        let sign = signs[query[0]];
        let b = &p[query[1] - 1];

        if swaps[query[0]] {
            println!("{} {}", b[1] * sign[0] + num[0], b[0] * sign[1] + num[1])
        } else {
            println!("{} {}", b[0] * sign[0] + num[0], b[1] * sign[1] + num[1])
        }
    }
}
