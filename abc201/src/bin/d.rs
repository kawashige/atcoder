use std::collections::HashMap;

use proconio::input;
use proconio::marker::Chars;

fn dfs(
    a: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    memo: &mut HashMap<(usize, usize), (i32, i32)>,
) -> (i32, i32) {
    if i == a.len() - 1 && j == a[0].len() - 1 {
        return (0, 0);
    }

    if let Some((x, y)) = memo.get(&(i, j)) {
        return (*x, *y);
    }

    let (mut r, r_p) = if i < a.len() - 1 && j < a[0].len() - 1 {
        let down = dfs(a, i + 1, j, memo);
        let down_p = if a[i + 1][j] == '+' { 1 } else { -1 };
        let right = dfs(a, i, j + 1, memo);
        let right_p = if a[i][j + 1] == '+' { 1 } else { -1 };
        if (i + j) % 2 == 0 {
            if down.0 + down_p - down.1 > right.0 + right_p - right.1 {
                (down, down_p)
            } else {
                (right, right_p)
            }
        } else {
            if down.1 + down_p - down.0 > right.1 + right_p - right.0 {
                (down, down_p)
            } else {
                (right, right_p)
            }
        }
    } else if i < a.len() - 1 {
        let down = dfs(a, i + 1, j, memo);
        let down_p = if a[i + 1][j] == '+' { 1 } else { -1 };
        (down, down_p)
    } else {
        let right = dfs(a, i, j + 1, memo);
        let right_p = if a[i][j + 1] == '+' { 1 } else { -1 };
        (right, right_p)
    };
    if (i + j) % 2 == 0 {
        r.0 += r_p;
    } else {
        r.1 += r_p;
    }

    memo.insert((i, j), r);
    r
}

fn main() {
    input! {
        h: usize,
        _w: usize,
        a: [Chars; h]
    }

    let r = dfs(&a, 0, 0, &mut HashMap::new());

    if r.0 == r.1 {
        println!("Draw");
    } else if r.0 > r.1 {
        println!("Takahashi");
    } else {
        println!("Aoki");
    }
}
