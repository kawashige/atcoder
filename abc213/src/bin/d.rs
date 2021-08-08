use proconio::input;

fn dfs(i: usize, list: &Vec<Vec<usize>>, seen: &mut Vec<bool>) {
    println!("{}", i + 1);

    seen[i] = true;

    for next in &list[i] {
        if seen[*next] {
            continue;
        }
        dfs(*next, list, seen);

        println!("{}", i + 1)
    }
}

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n - 1]
    }

    let mut list = vec![vec![]; n];
    for (a, b) in ab {
        list[a - 1].push(b - 1);
        list[b - 1].push(a - 1);
    }
    for i in 0..n {
        list[i].sort_unstable();
    }

    dfs(0, &list, &mut vec![false; n]);
}
