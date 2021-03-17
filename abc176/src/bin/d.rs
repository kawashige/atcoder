use std::collections::HashSet;

use proconio::input;
use proconio::marker::Chars;

fn color(
    s: &Vec<Vec<char>>,
    board: &mut Vec<Vec<i32>>,
    i: usize,
    j: usize,
    g: i32,
    e: &mut HashSet<(usize, usize)>,
) {
    board[i + 2][j + 2] = g;
    if 0 < i {
        if s[i - 1][j] == '.' && board[i - 1 + 2][j + 2] == -1 {
            color(s, board, i - 1, j, g, e);
        } else if s[i - 1][j] == '#' {
            e.insert((i, j));
        }
    }
    if 0 < j {
        if s[i][j - 1] == '.' && board[i + 2][j - 1 + 2] == -1 {
            color(s, board, i, j - 1, g, e);
        } else if s[i][j - 1] == '#' {
            e.insert((i, j));
        }
    }
    if i < s.len() - 1 {
        if s[i + 1][j] == '.' && board[i + 1 + 2][j + 2] == -1 {
            color(s, board, i + 1, j, g, e);
        } else if s[i + 1][j] == '#' {
            e.insert((i, j));
        }
    }
    if j < s[0].len() - 1 {
        if s[i][j + 1] == '.' && board[i + 2][j + 1 + 2] == -1 {
            color(s, board, i, j + 1, g, e);
        } else if s[i][j + 1] == '#' {
            e.insert((i, j));
        }
    }
}

fn main() {
    input! {
        h: usize,
        w: usize,
        ch: usize,
        cw: usize,
        dh: usize,
        dw: usize,
        mut s: [Chars; h]
    }

    let mut edges = Vec::new();
    let mut board = vec![vec![-1; w + 4]; h + 4];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' && board[i + 2][j + 2] == -1 {
                let mut e = HashSet::new();
                color(&s, &mut board, i, j, edges.len() as i32, &mut e);
                edges.push(e);
            }
        }
    }

    let start = board[ch + 1][cw + 1];
    let end = board[dh + 1][dw + 1];

    if start == end {
        println!("0");
        return;
    }

    let mut count = 0;
    let mut next = vec![start as usize];
    let mut seen = vec![false; edges.len()];
    while !next.is_empty() {
        count += 1;
        let mut new_next = Vec::new();
        for n in next {
            if seen[n] {
                continue;
            }
            seen[n] = true;
            for e in &edges[n] {
                for i in -2_i32..=2 {
                    for j in -2_i32..=2 {
                        let g = board[(e.0 as i32 + i + 2) as usize][(e.1 as i32 + j + 2) as usize];

                        if g != n as i32 && g != -1 {
                            if g == end {
                                println!("{}", count);
                                return;
                            }
                            new_next.push(g as usize);
                        }
                    }
                }
            }
        }
        next = new_next;
    }

    println!("-1");
}
