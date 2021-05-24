use proconio::input;

fn main() {
    input! {
        n: usize,
        mut ab: [(usize, usize); n],
        mut cd: [(usize, usize); n]
    }

    cd.sort_unstable_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));

    let mut red = vec![vec![0; 2 * n]; 2 * n];
    for (a, b) in ab {
        red[a][b] += 1;
    }

    let mut count = 0;
    for (c, d) in cd {
        let mut found = false;
        for x in (0..c).rev() {
            for y in 0..d {
                if red[x][y] == 1 {
                    red[x][y] = 0;
                    count += 1;
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
    }

    println!("{}", count);
}
