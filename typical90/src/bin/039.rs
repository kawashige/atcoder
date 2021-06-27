use proconio::input;

fn dfs(i: usize, d: i32, list: &Vec<Vec<usize>>, count: &mut Vec<usize>, depth: &mut Vec<i32>) {
    depth[i] = d;

    let mut c = 1;
    for child in &list[i] {
        if depth[*child] != -1 {
            continue;
        }
        dfs(*child, d + 1, list, count, depth);
        c += count[*child];
    }
    count[i] = c;
}

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n - 1]
    }

    let mut list = vec![vec![]; n];
    for (a, b) in &ab {
        list[a - 1].push(b - 1);
        list[b - 1].push(a - 1);
    }

    let mut count = vec![0; n];
    let mut depth = vec![-1; n];

    dfs(0, 0, &list, &mut count, &mut depth);

    let mut r = 0;
    for (a, b) in ab {
        r += if depth[a - 1] < depth[b - 1] {
            count[b - 1] * (n - count[b - 1])
        } else {
            count[a - 1] * (n - count[a - 1])
        }
    }

    println!("{}", r);
}
