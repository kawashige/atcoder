use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    }

    let mut list = vec![vec![]; n];
    for (a, b) in ab {
        list[a - 1].push(b - 1);
    }

    let mut count = 0;
    for i in 0..n {
        let mut seen = vec![false; n];
        let mut stack = vec![i];
        while let Some(node) = stack.pop() {
            if seen[node] {
                continue;
            }

            seen[node] = true;
            count += 1;

            for next in &list[node] {
                if seen[*next] {
                    continue;
                }
                stack.push(*next);
            }
        }
    }

    println!("{}", count);
}
