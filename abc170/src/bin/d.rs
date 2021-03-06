use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut heap = BinaryHeap::new();
    let mut set = HashSet::new();

    let mut max = 0;
    for num in a {
        heap.push(Reverse(num));
        max = std::cmp::max(max, num);
    }

    let mut result = 0;
    while let Some(Reverse(num)) = heap.pop() {
        let mut duplicated = false;
        if !heap.is_empty() && heap.peek().unwrap() == &Reverse(num) {
            duplicated = true;
            while !heap.is_empty() && heap.peek().unwrap() == &Reverse(num) {
                heap.pop();
            }
        }

        if set.contains(&num) {
            continue;
        }

        for i in ((num * 2)..=max).step_by(num) {
            set.insert(i);
        }

        if !duplicated {
            result += 1;
        }
    }

    println!("{}", result);
}
