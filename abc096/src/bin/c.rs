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
    let board = board;

    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if board[i][j] == '.' {
                continue;
            }
            if (i == 0 || board[i - 1][j] == '.')
                && (j == 0 || board[i][j - 1] == '.')
                && (i == h - 1 || board[i + 1][j] == '.')
                && (j == w - 1 || board[i][j + 1] == '.')
            {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
