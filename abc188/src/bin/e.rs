use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i32; n],
        roads: [(usize, usize); m]
    }

    let mut lists = vec![vec![]; n];
    for (f, t) in roads {
        lists[f - 1].push(t - 1);
    }

    let mut min_costs = vec![None; n];
    let mut max = std::i32::MIN;
    for i in 0..n {
        let mut min = a[i];
        if let Some(v) = min_costs[i] {
            max = std::cmp::max(max, a[i] - v);
            min = std::cmp::min(min, v);
        }
        for j in &lists[i] {
            if min_costs[*j].is_none() || Some(min) < min_costs[*j] {
                min_costs[*j] = Some(min);
            }
        }
    }

    println!("{}", max);
}
