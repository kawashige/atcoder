use std::collections::VecDeque;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h]
    }

    for i in 0..h {
        for j in 0..w {
            if c[i][j] == 's' {
                let mut queue = VecDeque::new();
                queue.push_back(((i, j), 0));

                let mut seen = vec![vec![false; w]; h];

                while let Some(((x, y), count)) = queue.pop_front() {
                    if c[x][y] == 'g' {
                        println!("YES");
                        return;
                    }

                    if seen[x][y] {
                        continue;
                    }
                    seen[x][y] = true;

                    for (dx, dy) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
                        let (x2, y2) = (x as i32 + dx, y as i32 + dy);
                        if x2 < 0
                            || h as i32 <= x2
                            || y2 < 0
                            || w as i32 <= y2
                            || seen[x2 as usize][y2 as usize]
                            || (count == 2 && c[x2 as usize][y2 as usize] == '#')
                        {
                            continue;
                        }

                        if c[x2 as usize][y2 as usize] == '#' {
                            queue.push_back(((x2 as usize, y2 as usize), count + 1));
                        } else {
                            queue.push_front(((x2 as usize, y2 as usize), count));
                        }
                    }
                }
            }
        }
    }

    println!("NO")
}
