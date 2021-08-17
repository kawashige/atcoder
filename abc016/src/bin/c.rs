use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    }

    let mut friend = vec![vec![false; n]; n];

    for (a, b) in ab {
        friend[a - 1][b - 1] = true;
        friend[b - 1][a - 1] = true;
    }

    for i in 0..n {
        let mut found = vec![false; n];
        for j in 0..n {
            if i == j || !friend[i][j] {
                continue;
            }
            for k in 0..n {
                if i == k || j == k || friend[i][k] || !friend[j][k] {
                    continue;
                }
                found[k] = true;
            }
        }
        println!("{}", found.into_iter().filter(|x| *x).count());
    }
}
