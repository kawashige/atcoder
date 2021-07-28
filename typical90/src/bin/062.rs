use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n]
    }

    let mut used = vec![false; n];
    let mut edge = vec![vec![]; n];

    let mut target = Vec::new();
    let mut result = Vec::new();

    for i in 0..n {
        if ab[i].0 - 1 == i || ab[i].1 - 1 == i {
            target.push(i);
            result.push(i);
            used[i] = true;
        }
        edge[ab[i].0 - 1].push(i);
        edge[ab[i].1 - 1].push(i);
    }

    while let Some(v) = target.pop() {
        for next in &edge[v] {
            if used[*next] {
                continue;
            }
            used[*next] = true;
            target.push(*next);
            result.push(*next);
        }
    }

    if result.len() == n {
        for x in result.iter().rev() {
            println!("{}", x + 1);
        }
    } else {
        println!("-1");
    }
}
