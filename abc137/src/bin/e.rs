use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        p: i32,
        abc: [(usize, usize, i32); m]
    }

    let mut list = vec![vec![]; n];
    for (a, b, c) in abc {
        list[a - 1].push((b - 1, p - c));
    }

    let mut d = vec![std::i32::MAX; n];

    d[0] = 0;
    for i in 0..n {
        let mut nodes = Vec::new();
        for j in 0..n {
            for (b, c) in &list[j] {
                if d[j] != std::i32::MAX && d[*b] > d[j] + c {
                    d[*b] = d[j] + c;
                    if i == n - 1 {
                        nodes.push(*b);
                    }
                }
            }
        }

        let mut seen = vec![false; n];
        let mut found = false;
        while let Some(v) = nodes.pop() {
            if v == n - 1 {
                found = true;
                break;
            }
            seen[v] = true;
            for (next, _) in &list[v] {
                if seen[*next] {
                    continue;
                }
                nodes.push(*next);
            }
        }

        if found {
            println!("-1");
            return;
        }
    }

    println!("{}", (-d[n - 1]).max(0));
}
