use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
        q: usize,
        xy: [(usize, usize); q],
    }

    let mut list = vec![vec![]; n];
    for (a, b) in ab {
        list[a - 1].push(b - 1);
        list[b - 1].push(a - 1);
    }

    let b = (2.0 * m as f64).sqrt() as usize;

    let mut large = Vec::new();
    for i in 0..n {
        if list[i].len() >= b {
            large.push(i);
        }
    }

    let mut link = vec![vec![false; large.len()]; n];

    for i in 0..large.len() {
        for j in &list[large[i]] {
            link[*j][i] = true;
        }
        link[large[i]][i] = true;
    }

    let mut update = vec![-1; n];
    let mut update_large = vec![-1; large.len()];

    for i in 0..q {
        let mut last = update[xy[i].0 - 1];
        for j in 0..large.len() {
            if link[xy[i].0 - 1][j] {
                last = std::cmp::max(last, update_large[j]);
            }
        }

        println!("{}", if last != -1 { xy[last as usize].1 } else { 1 });

        if list[xy[i].0 - 1].len() < b {
            update[xy[i].0 - 1] = i as i32;
            for j in &list[xy[i].0 - 1] {
                update[*j] = i as i32;
            }
        } else {
            let prt = large.binary_search(&(xy[i].0 - 1)).unwrap();
            update_large[prt] = i as i32;
        }
    }
}
