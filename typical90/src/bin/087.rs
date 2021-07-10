use proconio::input;

fn count_pair(v: i64, a: &Vec<Vec<i64>>, n: usize, p: usize) -> usize {
    let mut matrix = a.clone();
    for i in 0..n {
        for j in 0..n {
            if matrix[i][j] == -1 {
                matrix[i][j] = v;
            }
        }
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if matrix[i][k] != std::i64::MAX
                    && matrix[k][j] != std::i64::MAX
                    && matrix[i][j] > matrix[i][k] + matrix[k][j]
                {
                    matrix[i][j] = matrix[i][k] + matrix[k][j];
                }
            }
        }
    }

    let mut count = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            if matrix[i][j] <= p as i64 {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    input! {
        n: usize,
        p: usize,
        k: usize,
        mut a: [[i64; n]; n]
    }

    let count = count_pair(std::i64::MAX, &a, n, p);
    if count == k {
        println!("Infinity");
        return;
    } else if count > k {
        println!("0");
        return;
    }

    let count_1 = count_pair(1, &a, n, p);
    if count_1 < k {
        println!("0");
        return;
    }

    let mut ok = 1;
    let mut ng = p + 1;

    while ok + 1 < ng {
        let mid = (ok + ng) / 2;
        let count = count_pair(mid as i64, &a, n, p);

        if count >= k {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    if count_pair(ok as i64, &a, n, p) != k {
        println!("0");
        return;
    }

    if count_1 == k {
        println!("{}", ok);
        return;
    }

    let mut ng2 = 1;
    let mut ok2 = ok;

    while ng2 + 1 < ok2 {
        let mid = (ok2 + ng2) / 2;
        let count = count_pair(mid as i64, &a, n, p);

        if count <= k {
            ok2 = mid;
        } else {
            ng2 = mid;
        }
    }

    println!("{}", ok - ng2);
}
