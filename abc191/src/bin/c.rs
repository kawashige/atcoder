use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        grid: [Chars; h]
    }

    let mut result = 0;
    for i in 0..(h - 1) {
        for j in 0..(w - 1) {
            let mut count = 0;
            for (x, y) in [(i, j), (i + 1, j), (i, j + 1), (i + 1, j + 1)].iter() {
                if grid[*x][*y] == '#' {
                    count += 1;
                }
            }
            if count == 1 || count == 3 {
                result += 1;
            }
        }
    }
    println!("{}", result);
}
