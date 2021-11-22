use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n]
    }

    let mut stack = vec![x - 1];
    let mut seen = vec![false; n];

    while let Some(node) = stack.pop() {
        if seen[node] {
            false;
        }
        seen[node] = true;

        if !seen[a[node] - 1] {
            stack.push(a[node] - 1);
        }
    }

    println!("{}", seen.iter().filter(|b| **b).count());
}
