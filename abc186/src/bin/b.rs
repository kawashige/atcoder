use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        grid: [[usize; w]; h]
    }

    let mut min = 101;
    for i in 0..h {
        for j in 0..w {
            min = std::cmp::min(min, grid[i][j]);
        }
    }

    let mut count = 0;
    for i in 0..h {
        for j in 0..w {
            count += grid[i][j] - min;
        }
    }

    println!("{}", count);
}
