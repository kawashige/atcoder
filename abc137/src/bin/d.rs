use proconio::input;
use std::collections::BinaryHeap;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); n]
    }

    let mut map = HashMap::new();
    for (a, b) in ab {
        if a > m {
            continue;
        }
        (*map.entry(m - a).or_insert(Vec::new())).push(b);
    }

    let mut heap = BinaryHeap::new();
    let mut sum = 0;
    for i in (0..m).rev() {
        if let Some(bb) = map.get(&i) {
            for b in bb {
                heap.push(b);
            }
        }
        if let Some(b) = heap.pop() {
            sum += b;
        }
    }

    println!("{}", sum);
}
