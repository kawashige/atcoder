use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        xy: [(usize, usize); m]
    }

    let mut matrix = vec![vec![false; n]; n];
    for (x, y) in xy {
        matrix[x - 1][y - 1] = true;
        matrix[y - 1][x - 1] = true;
    }

    let mut r = 0;
    for i in 0..2_usize.pow(n as u32) {
        let idx = (0..n)
            .filter(|j| i & (1 << j) as usize > 0)
            .collect::<Vec<_>>();
        let mut ok = true;
        for j in 0..idx.len() {
            for k in (j + 1)..idx.len() {
                if !matrix[idx[j]][idx[k]] {
                    ok = false;
                    break;
                }
            }
            if !ok {
                break;
            }
        }

        if ok {
            r = r.max(idx.len());
        }
    }

    println!("{}", r);
}
