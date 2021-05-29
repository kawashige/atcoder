use proconio::input;

fn dfs(
    i: usize,
    prev: usize,
    sum: i32,
    a: &Vec<Vec<i32>>,
    d: &Vec<Vec<bool>>,
    seen: &mut Vec<bool>,
    min: &mut i32,
) {
    if i == a.len() {
        if *min == -1 || sum < *min {
            *min = sum;
        }
        return;
    }

    for j in 0..a.len() {
        if seen[j] || d[prev][j] {
            continue;
        }

        seen[j] = true;
        dfs(i + 1, j, sum + a[j][i], a, d, seen, min);
        seen[j] = false;
    }
}

fn main() {
    input! {
        n: usize,
        a: [[i32; n]; n],
        m: usize,
        xy: [(usize, usize); m]
    }

    let mut d = vec![vec![false; n]; n];
    for (x, y) in xy {
        d[x - 1][y - 1] = true;
        d[y - 1][x - 1] = true;
    }

    let mut seen = vec![false; n];
    let mut min = -1;
    for i in 0..n {
        seen[i] = true;
        dfs(1, i, a[i][0], &a, &d, &mut seen, &mut min);
        seen[i] = false;
    }

    println!("{}", min);
}
