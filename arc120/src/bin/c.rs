use proconio::input;
use std::collections::HashMap;
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
        a: [usize; n],
        b: [usize; n],
    }

    let mut map = HashMap::new();
    for i in (0..n).rev() {
        (*map.entry(i + a[i]).or_insert(Vec::new())).push(i);
    }

    let mut bit = Bit::new(n + 1);

    let mut count = 0;
    for i in 0..n {
        if let Some(indices) = map.get_mut(&(i + b[i])) {
            if let Some(j) = indices.pop() {
                let k = j - bit.sum(j);
                bit.add(j + 1, 1);
                count += k;
            } else {
                println!("-1");
                return;
            }
        } else {
            println!("-1");
            return;
        }
    }

    println!("{}", count);
}
