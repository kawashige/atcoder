use proconio::input;

fn main() {
    input! {
        n: usize,
        lr: [[usize; 4]; n]
    }

    let mut grid = vec![vec![0_i32; 1001]; 1001];
    for i in 0..n {
        grid[lr[i][0]][lr[i][1]] += 1;
        grid[lr[i][2]][lr[i][3]] += 1;
        grid[lr[i][2]][lr[i][1]] -= 1;
        grid[lr[i][0]][lr[i][3]] -= 1;
    }

    for i in 1..1001 {
        for j in 0..1001 {
            grid[i][j] += grid[i - 1][j];
        }
    }

    for i in 0..1001 {
        for j in 1..1001 {
            grid[i][j] += grid[i][j - 1];
        }
    }

    let mut count = vec![0; n + 1];
    for i in 0..1001 {
        for j in 0..1001 {
            count[grid[i][j] as usize] += 1;
        }
    }

    for i in 1..=n {
        println!("{}", count[i]);
    }
}
