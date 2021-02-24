use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        m: usize,
        mut lights: [(usize, usize); n],
        mut blocks: [(usize, usize); m],
    }

    let mut grid = vec![vec![0; w]; h];

    for (i, j) in lights {
        grid[i - 1][j - 1] = 1;
    }

    for (i, j) in blocks {
        grid[i - 1][j - 1] = 2;
    }

    for i in 0..h {
        let mut light = false;
        for j in 0..w {
            if grid[i][j] == 1 {
                light = true;
            } else if grid[i][j] == 2 {
                light = false;
            } else if light {
                grid[i][j] = 3;
            }
        }
    }

    for i in (0..h).rev() {
        let mut light = false;
        for j in (0..w).rev() {
            if grid[i][j] == 1 {
                light = true;
            } else if grid[i][j] == 2 {
                light = false;
            } else if light {
                grid[i][j] = 3;
            }
        }
    }

    for j in 0..w {
        let mut light = false;
        for i in 0..h {
            if grid[i][j] == 1 {
                light = true;
            } else if grid[i][j] == 2 {
                light = false;
            } else if light {
                grid[i][j] = 3;
            }
        }
    }

    for j in (0..w).rev() {
        let mut light = false;
        for i in (0..h).rev() {
            if grid[i][j] == 1 {
                light = true;
            } else if grid[i][j] == 2 {
                light = false;
            } else if light {
                grid[i][j] = 3;
            }
        }
    }

    let mut count = 0;
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == 1 || grid[i][j] == 3 {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
