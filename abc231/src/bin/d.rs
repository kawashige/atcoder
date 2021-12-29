use proconio::input;

fn dfs(i: usize, parent: usize, list: &Vec<Vec<usize>>, seen: &mut Vec<bool>) -> bool {
    seen[i] = true;
    for next in &list[i] {
        if next == &parent {
            continue;
        }

        if seen[*next] {
            return false;
        }

        if !dfs(*next, i, list, seen) {
            return false;
        }
    }

    true
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    }

    let mut list = vec![vec![]; n];
    for (a, b) in ab {
        list[a - 1].push(b - 1);
        list[b - 1].push(a - 1);
    }

    if list.iter().any(|x| x.len() > 2) {
        println!("No");
        return;
    }

    let mut seen = vec![false; n];

    for i in 0..n {
        if !seen[i] {
            if !dfs(i, n, &list, &mut seen) {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
