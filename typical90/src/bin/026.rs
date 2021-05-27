use proconio::input;

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

    let mut v = vec![vec![]; 2];
    let mut seen = vec![false; n];
    let mut stack = vec![(0, 1)];
    while let Some((node, depth)) = stack.pop() {
        seen[node] = true;
        v[depth % 2].push(node);

        for next in &list[node] {
            if seen[*next] {
                continue;
            }
            stack.push((*next, depth + 1));
        }
    }

    let i = if v[0].len() > v[1].len() { 0 } else { 1 };
    println!(
        "{}",
        v[i].iter()
            .take(n / 2)
            .map(|x| (x + 1).to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
