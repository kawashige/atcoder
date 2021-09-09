use proconio::input;

fn dfs(i: usize, list: &Vec<Vec<usize>>, seen: &mut Vec<bool>, nodes: &mut Vec<usize>) {
    nodes.push(i);
    seen[i] = true;

    for next in &list[i] {
        if seen[*next] {
            continue;
        }
        dfs(*next, list, seen, nodes);
    }
}

fn dfs2(nodes: &Vec<usize>, colors: &mut Vec<usize>, matrix: &Vec<Vec<bool>>, r: &mut u64) {
    if nodes.len() == colors.len() {
        *r += 1;
        return;
    }

    for j in 0..3 {
        if (0..colors.len()).any(|k| matrix[nodes[colors.len()]][nodes[k]] && colors[k] == j) {
            continue;
        }

        colors.push(j);

        dfs2(nodes, colors, matrix, r);

        colors.pop();
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    }

    let mut matrix = vec![vec![false; n]; n];
    let mut lists = vec![vec![]; n];
    for (a, b) in ab {
        matrix[a - 1][b - 1] = true;
        matrix[b - 1][a - 1] = true;

        lists[a - 1].push(b - 1);
        lists[b - 1].push(a - 1);
    }

    let mut seen = vec![false; n];
    let mut r: u64 = 1;

    for i in 0..n {
        if seen[i] {
            continue;
        }
        let mut nodes = Vec::new();
        dfs(i, &lists, &mut seen, &mut nodes);
        let mut x = 0;
        dfs2(&nodes, &mut Vec::new(), &matrix, &mut x);
        r *= x;
    }

    println!("{}", r);
}
