use proconio::fastout;
use proconio::input;

fn dfs(
    i: usize,
    d: usize,
    list: &Vec<Vec<usize>>,
    query: &Vec<Vec<(usize, usize)>>,
    result: &mut Vec<i32>,
    count: &mut Vec<usize>,
) {
    for (q, i) in &query[i] {
        result[*i] = -(count[*q] as i32);
    }

    for next in &list[i] {
        dfs(*next, d + 1, list, query, result, count);
    }

    count[d] += 1;

    for (q, i) in &query[i] {
        result[*i] += count[*q] as i32;
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [usize; n - 1],
        q: usize,
        ud: [(usize, usize); q]
    }

    let mut query = vec![vec![]; n];
    for (i, (u, d)) in ud.into_iter().enumerate() {
        query[u - 1].push((d, i));
    }

    let mut list = vec![vec![]; n];
    for i in 0..p.len() {
        list[p[i] - 1].push(i + 1);
    }

    let mut result = vec![0; q];
    let mut count = vec![0; n];

    dfs(0, 0, &list, &query, &mut result, &mut count);

    for x in result {
        println!("{:?}", x);
    }
}
