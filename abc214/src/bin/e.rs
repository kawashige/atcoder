use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        t: usize
    }

    for _ in 0..t {
        input! {
            n: usize,
            mut lr: [(usize, usize); n]
        }

        lr.sort_unstable();

        let mut i = 0;
        let mut x = lr[0].0;
        let mut heap = BinaryHeap::new();
        let mut ok = true;
        while ok && i < n {
            while i < n && lr[i].0 <= x {
                heap.push(Reverse(lr[i].1));
                i += 1;
            }

            while let Some(Reverse(y)) = heap.pop() {
                if y < x {
                    ok = false;
                    break;
                }

                x += 1;
                while i < n && lr[i].0 <= x {
                    heap.push(Reverse(lr[i].1));
                    i += 1;
                }
            }

            if i < n {
                x = lr[i].0;
            }
        }

        if ok {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
