use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n - 1],
    }

    let mut list = vec![vec![]; n];
    for (a, b) in ab {
        list[a - 1].push(b - 1);
        list[b - 1].push(a - 1);
    }

    let mut f_depth = vec![n; n];
    let mut stack = vec![(0, 0)];
    while let Some((node, depth)) = stack.pop() {
        f_depth[node] = depth;

        for next in &list[node] {
            if f_depth[*next] != n {
                continue;
            }
            stack.push((*next, depth + 1));
        }
    }

    let mut s_depth = vec![n; n];
    let mut stack = vec![(n - 1, 0)];
    while let Some((node, depth)) = stack.pop() {
        s_depth[node] = depth;

        for next in &list[node] {
            if s_depth[*next] != n {
                continue;
            }
            stack.push((*next, depth + 1));
        }
    }

    let f_count = (0..n).filter(|i| f_depth[*i] <= s_depth[*i]).count();

    if f_count > n - f_count {
        println!("Fennec");
    } else {
        println!("Snuke");
    }
}
