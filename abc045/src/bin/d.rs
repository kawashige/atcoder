use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        h: u64,
        w: u64,
        n: usize,
        ab: [(u64, u64); n]
    }

    let mut map = HashMap::new();
    for (a, b) in ab {
        for i in &[-2, -1, 0] {
            for j in &[-2, -1, 0] {
                let (x, y) = (a as i32 + i, b as i32 + j);
                if x > 0 && x + 2 <= h as i32 && y > 0 && y + 2 <= w as i32 {
                    *map.entry((x, y)).or_insert(0) += 1;
                }
            }
        }
    }

    let mut count = vec![0; 10];

    for (_, v) in map {
        count[v as usize] += 1;
    }

    count[0] = (h - 2) * (w - 2);
    for i in 1..10 {
        count[0] -= count[i];
    }

    for c in count {
        println!("{}", c);
    }
}
