use proconio::input;

fn main() {
    input! {
        n: usize,
        mut xy: [(i64, i64); n]
    }

    xy.sort_unstable();

    let mut count = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            for k in (j + 1)..n {
                if (xy[i].0 == xy[j].0 && xy[i].0 == xy[k].0)
                    || (xy[i].1 == xy[j].1 && xy[i].1 == xy[k].1)
                    || (xy[j].1 - xy[i].1) * (xy[k].0 - xy[j].0)
                        == (xy[j].0 - xy[i].0) * (xy[k].1 - xy[j].1)
                {
                    continue;
                }
                count += 1;
            }
        }
    }

    println!("{}", count);
}
