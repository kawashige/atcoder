use proconio::input;

fn dfs(i: usize, list: &Vec<Vec<usize>>) -> usize {
    if list[i].is_empty() {
        return 1;
    }

    let mut min = std::usize::MAX;
    let mut max = 0;
    for buka in &list[i] {
        let x = dfs(*buka, list);
        min = min.min(x);
        max = max.max(x);
    }

    min + max + 1
}

fn main() {
    input! {
        n: usize,
        b: [usize; n - 1]
    }

    let mut list = vec![vec![]; n];
    for i in 0..(n - 1) {
        list[b[i] - 1].push(i + 1);
    }

    println!("{}", dfs(0, &list));
}
