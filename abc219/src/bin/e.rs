use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        a: [[usize; 4]; 4]
    }

    let mut house: usize = 0;
    for i in 0..4 {
        for j in 0..4 {
            if a[i][j] == 1 {
                house |= 1 << 4 * i + j;
            }
        }
    }
    let count = house.count_ones();
    let mut result = 0;

    for i in 1..2_usize.pow(16) {
        if (i & house).count_ones() != count {
            continue;
        }

        let mut board = [[0; 4]; 4];
        let mut house_p = None;
        for j in 0..16 {
            if i & 1 << j > 0 {
                board[j / 4][j % 4] = 1;
                if house_p.is_none() {
                    house_p = Some((j / 4, j % 4));
                }
            }
        }

        let mut deque = VecDeque::new();
        deque.push_back(house_p.unwrap());
        let mut seen = 0;

        while let Some((c, r)) = deque.pop_front() {
            if board[c][r] == 2 {
                continue;
            }
            seen += 1;
            board[c][r] = 2;

            if c > 0 && board[c - 1][r] == 1 {
                deque.push_back((c - 1, r));
            }
            if r > 0 && board[c][r - 1] == 1 {
                deque.push_back((c, r - 1));
            }
            if c < 3 && board[c + 1][r] == 1 {
                deque.push_back((c + 1, r));
            }
            if r < 3 && board[c][r + 1] == 1 {
                deque.push_back((c, r + 1));
            }
        }

        let mut no_inside = true;
        for (c, r) in [(1, 1), (1, 2), (2, 1), (2, 2)].iter() {
            if board[*c][*r] == 0 {
                let mut deque = VecDeque::new();
                deque.push_back((*c, *r));
                let mut seen = [[false; 4]; 4];
                let mut inside = true;
                while let Some((c, r)) = deque.pop_front() {
                    if seen[c][r] {
                        continue;
                    }
                    if c == 0 || c == 3 || r == 0 || r == 3 {
                        inside = false;
                        break;
                    }
                    seen[c][r] = true;

                    if c > 0 && board[c - 1][r] == 0 && !seen[c - 1][r] {
                        deque.push_back((c - 1, r));
                    }
                    if r > 0 && board[c][r - 1] == 0 && !seen[c][r - 1] {
                        deque.push_back((c, r - 1));
                    }
                    if c < 3 && board[c + 1][r] == 0 && !seen[c + 1][r] {
                        deque.push_back((c + 1, r));
                    }
                    if r < 3 && board[c][r + 1] == 0 && !seen[c][r + 1] {
                        deque.push_back((c, r + 1));
                    }
                }

                if inside {
                    no_inside = false;
                    break;
                }
            }
        }

        if seen == i.count_ones() && no_inside {
            result += 1;
        }
    }

    println!("{}", result);
}
