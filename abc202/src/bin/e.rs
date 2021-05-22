use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n - 1],
        q: usize,
        _ud: [(usize, usize); q]
    }

    let mut list = vec![vec![]; n];
    for i in 0..p.len() {
        list[p[i] - 1].push(i + 1);
    }

    let mut d = vec![0; n];
    let mut stack = vec![0];
    while let Some(node) = stack.pop() {
        for next in &list[node] {
            d[*next] = d[node] + 1;
            stack.push(*next);
        }
    }

    println!("{:?}", d);
}
