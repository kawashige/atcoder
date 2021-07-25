use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h]
    }

    let mut result = vec![vec!['.'; w]; h];

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                continue;
            }
            if [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ]
            .iter()
            .all(|(r, c)| {
                let (x, y) = (i as i32 + r, j as i32 + c);
                x < 0 || y < 0 || x >= h as i32 || y >= w as i32 || s[x as usize][y as usize] == '#'
            }) {
                result[i][j] = '#';
            }
        }
    }

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                continue;
            }

            if [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 0),
                (0, 1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ]
            .iter()
            .any(|(r, c)| {
                let (x, y) = (i as i32 + r, j as i32 + c);
                x >= 0
                    && y >= 0
                    && x < h as i32
                    && y < w as i32
                    && result[x as usize][y as usize] == '#'
            }) {
                continue;
            }

            println!("impossible");
            return;
        }
    }

    println!("possible");
    for row in result {
        println!("{}", row.into_iter().collect::<String>());
    }
}
