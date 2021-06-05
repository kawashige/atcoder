use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[usize; n - 1]; n]
    }

    let mut indices = vec![1; n];
    let mut seen = vec![vec![0; n]; n];
    let mut next = Vec::new();
    for i in 0..n {
        next.push((i, a[i][0] - 1));
    }

    let mut count = 0;
    let mut d = 0;
    while !next.is_empty() {
        d += 1;
        let mut new = Vec::new();
        for (i, j) in next {
            seen[i][j] += 1;
            seen[j][i] += 1;
            if seen[i][j] == 2 {
                count += 2;
                if indices[i] < n - 1 {
                    new.push((i, a[i][indices[i]] - 1));
                    indices[i] += 1;
                }
                if indices[j] < n - 1 {
                    new.push((j, a[j][indices[j]] - 1));
                    indices[j] += 1;
                }
            }
        }
        next = new;
    }

    if count == n * (n - 1) {
        println!("{}", d);
    } else {
        println!("-1");
    }
}
