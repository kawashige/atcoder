use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        p: [[usize; w]; h]
    }

    let mut max = 0;

    for i in 0..2_usize.pow(h as u32) {
        let mut map = HashMap::new();
        for j in 0..w {
            let mut x = 0;
            for k in 0..h {
                if i & 1 << k == 0 {
                    continue;
                }
                if x == 0 {
                    x = p[k][j];
                } else if x != p[k][j] {
                    x = 0;
                    break;
                }
            }

            if x > 0 {
                *map.entry(x).or_insert(0) += 1;
            }
        }

        let mut counts = map.values().collect::<Vec<&usize>>();
        counts.sort_unstable();

        max = std::cmp::max(max, *counts.last().unwrap_or(&&0) * i.count_ones() as usize);
    }

    println!("{}", max);
}
