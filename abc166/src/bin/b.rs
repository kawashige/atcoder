use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        sweets: [[usize]; k]
    }

    let set = sweets
        .into_iter()
        .flatten()
        .collect::<std::collections::HashSet<usize>>();
    println!("{}", n - set.len());
}
