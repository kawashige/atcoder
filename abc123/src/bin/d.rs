use proconio::input;
use std::collections::BinaryHeap;

fn main() {
    input! {
        x: usize,
        y: usize,
        z: usize,
        mut k: usize,
        a: [u64; x],
        b: [u64; y],
        c: [u64; z],
    }

    let xy = x * y;
    let mut ab = Vec::with_capacity(x * y);

    for i in 0..x {
        for j in 0..y {
            ab.push(a[i] + b[j]);
        }
    }

    ab.sort_unstable_by(|a, b| b.cmp(&a));

    let mut heap = BinaryHeap::with_capacity(y);

    for i in 0..z {
        heap.push((c[i] + ab[0], 0));
    }

    while 0 < k {
        if let Some((n, i)) = heap.pop() {
            println!("{}", n);
            k -= 1;
            if i + 1 < xy {
                heap.push((n - ab[i] + ab[i + 1], i + 1));
            }
        }
    }
}
