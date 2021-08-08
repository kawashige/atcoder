use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        xy: [(i64, i64); n]
    }

    let mut d = vec![vec![0; n]; n];

    for i in 0..n {
        for j in (i + 1)..n {
            d[i][j] = (xy[i].0 - xy[j].0).pow(2) + (xy[i].1 - xy[j].1).pow(2);
            d[j][i] = d[i][j];
        }
    }

    let mut group = (0..n).map(|i| vec![i]).collect::<Vec<_>>();
    let mut max_d = 0;

    while group.len() > k {
        let mut min_d = std::i64::MAX;
        let mut min_g = (0, 0);
        let mut min_size = 0;

        for i in 0..group.len() {
            for j in (i + 1)..group.len() {
                let mut dist = std::i64::MIN;

                for k in 0..group[i].len() {
                    for l in 0..group[j].len() {
                        dist = dist.max(d[group[i][k]][group[j][l]]);
                    }
                }

                if dist < min_d || (dist == min_d && min_size < group[i].len() + group[j].len()) {
                    min_d = dist;
                    min_g = (i, j);
                    min_size = group[i].len() + group[j].len();
                }
            }
        }

        let mut g = group.remove(min_g.1);
        group[min_g.0].append(&mut g);
        max_d = min_d;
    }

    println!("{}", max_d);
}
