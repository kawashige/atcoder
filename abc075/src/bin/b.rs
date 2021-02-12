use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        board: [Chars; h],
    }

    let h: usize = h;
    let w: usize = w;
    let mut board = board;

    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if board[i][j] == '#' {
                continue;
            }
            let mut count = 0;
            for (x, y) in &[
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ] {
                if (i as i32 + x) < 0
                    || h as i32 - 1 < (i as i32 + x)
                    || (j as i32 + y) < 0
                    || w as i32 - 1 < (j as i32 + y)
                {
                    continue;
                }
                if board[(i as i32 + x) as usize][(j as i32 + y) as usize] == '#' {
                    count += 1;
                }
            }
            board[i][j] = count.to_string().chars().next().unwrap();
        }
    }
    for i in 0..h {
        println!("{}", board[i].iter().collect::<String>());
    }
}
