use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h]
    }

    let mut stack = vec![((0, 0), 1)];
    let mut dist = vec![vec![0; w]; h];
    while let Some(((i, j), d)) = stack.pop() {
        if d < dist[i][j] {
            continue;
        }
        dist[i][j] = d;

        if i + 1 < h && c[i + 1][j] == '.' && dist[i + 1][j] < d + 1 {
            stack.push(((i + 1, j), d + 1));
        }
        if j + 1 < w && c[i][j + 1] == '.' && dist[i][j + 1] < d + 1 {
            stack.push(((i, j + 1), d + 1));
        }
    }

    println!(
        "{}",
        dist.iter()
            .map(|row| row.iter().max().unwrap())
            .max()
            .unwrap()
    );
}
