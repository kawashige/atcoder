use proconio::input;

fn dfs(
    i: usize,
    women: usize,
    n: usize,
    m: usize,
    p: usize,
    q: usize,
    xyz: &Vec<(usize, usize, usize)>,
    max: &mut usize,
) {
    if n - i + (women.count_ones() as usize) < p {
        return;
    }

    if women.count_ones() as usize == p {
        let mut m = vec![0; m];
        for (x, y, z) in xyz {
            if women & 1 << (x - 1) > 0 {
                m[y - 1] += z;
            }
        }
        m.sort_unstable_by(|a, b| b.cmp(&a));
        let happy = m.iter().take(q).sum::<usize>();
        *max = std::cmp::max(*max, happy);
        return;
    }

    dfs(i + 1, women | 1 << i, n, m, p, q, xyz, max);
    dfs(i + 1, women, n, m, p, q, xyz, max);
}

fn main() {
    input! {
        n: usize,
        m: usize,
        p: usize,
        q: usize,
        r: usize,
        xyz: [(usize, usize, usize); r]
    }

    let mut max = 0;
    dfs(0, 0, n, m, p, q, &xyz, &mut max);
    println!("{}", max);
}
