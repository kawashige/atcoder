use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        heights: [usize; n],
        roads: [(usize, usize); m]
    }

    let mut adjacents: Vec<Vec<usize>> = vec![vec![]; n + 1];
    for (from, to) in roads {
        adjacents[from].push(heights[to - 1]);
        adjacents[to].push(heights[from - 1]);
    }

    let count = (1..=n)
        .filter(|i| adjacents[*i].is_empty() || adjacents[*i].iter().all(|a| a < &heights[i - 1]))
        .count();

    println!("{}", count);
}
