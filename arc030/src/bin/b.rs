use proconio::input;

fn dfs(i: usize, parent: usize, list: &Vec<Vec<usize>>, h: &Vec<usize>, r: &mut usize) -> bool {
    let mut found = h[i] > 0;
    for next in &list[i] {
        if next == &parent {
            continue;
        }

        if dfs(*next, i, list, h, r) {
            *r += 2;
            found = true;
        }
    }
    found
}

fn main() {
    input! {
        n: usize,
        x: usize,
        h: [usize; n],
        ab: [(usize, usize); n - 1]
    }

    let mut list = vec![vec![]; n];
    for (a, b) in ab {
        list[a - 1].push(b - 1);
        list[b - 1].push(a - 1);
    }

    let mut r = 0;
    dfs(x - 1, n, &list, &h, &mut r);

    println!("{}", r);
}
