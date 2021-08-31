use std::collections::VecDeque;

use proconio::input;
struct Bit {
    n: usize,
    pub data: Vec<usize>,
}

impl Bit {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            data: vec![0; n + 1],
        }
    }

    pub fn sum(&self, i: usize) -> usize {
        let mut r = 0;
        let mut i = i as i32;
        while i > 0 {
            r += self.data[i as usize];
            i -= i & -i;
        }
        r
    }

    pub fn add(&mut self, i: usize, x: usize) {
        let mut i = i as i32;
        while i as usize <= self.n {
            self.data[i as usize] += x;
            i += i & -i;
        }
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        mut lrx: [(usize, usize, usize); m]
    }

    lrx.sort_unstable_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.1)));

    let mut bit = Bit::new(n);
    let mut result = vec![0; n];

    let mut q = 1;
    let mut deque = VecDeque::new();

    for (l, r, x) in lrx {
        for i in q..=r {
            deque.push_back(i);
        }
        q = r + 1;

        let j = bit.sum(r) - bit.sum(l - 1);
        if x >= j {
            for _ in 0..(x - j) {
                let p = deque.pop_back().unwrap();
                result[p - 1] = 1;
                bit.add(p, 1);
            }
        }
    }

    for i in 0..n {
        if i > 0 {
            print!(" ")
        }
        print!("{}", result[i]);
    }
    print!("\n");
}
