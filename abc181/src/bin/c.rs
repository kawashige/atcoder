use proconio::input;

fn main() {
    input! {
        n: usize,
        mut pos: [(i32, i32); n]
    }

    pos.sort_unstable();

    for i in 0..n {
        for j in (i + 1)..n {
            for k in (j + 1)..n {
                if (pos[j].1 - pos[i].1) as f64 / (pos[j].0 - pos[i].0) as f64
                    == (pos[k].1 - pos[j].1) as f64 / (pos[k].0 - pos[j].0) as f64
                {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}
