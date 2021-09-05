use std::collections::{BTreeSet, HashMap};

use proconio::input;

struct MaxSegmentTree {
    pub node: Vec<usize>,
    pub n: usize,
}

impl MaxSegmentTree {
    pub fn new(nums: Vec<usize>) -> Self {
        let size = nums.len();

        let mut n = 1;
        while n < size {
            n *= 2;
        }
        let mut node = vec![0; 2 * n - 1];

        for i in 0..size {
            node[i + n - 1] = nums[i];
        }
        if n > 2 {
            for i in (0..=(n - 2)).rev() {
                node[i] = node[2 * i + 1].max(node[2 * i + 2]);
            }
        }

        Self { node, n }
    }

    pub fn update(&mut self, i: usize, v: usize) {
        let mut i = i + self.n - 1;

        self.node[i] = self.node[i].max(v);
        while i > 0 {
            i = (i - 1) / 2;
            self.node[i] = self.node[i].max(v);
        }
    }

    pub fn query(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> usize {
        if r <= a || b <= l {
            return 0;
        }
        if a <= l && r <= b {
            self.node[k]
        } else {
            let c1 = self.query(a, b, 2 * k + 1, l, (l + r) / 2);
            let c2 = self.query(a, b, 2 * k + 2, (l + r) / 2, r);
            c1.max(c2)
        }
    }
}
struct MinSegmentTree {
    pub node: Vec<usize>,
    pub n: usize,
}

impl MinSegmentTree {
    pub fn new(nums: Vec<usize>) -> Self {
        let size = nums.len();

        let mut n = 1;
        while n < size {
            n *= 2;
        }
        let mut node = vec![nums[0]; 2 * n - 1];

        for i in 0..size {
            node[i + n - 1] = nums[i];
        }
        if n > 2 {
            for i in (0..=(n - 2)).rev() {
                node[i] = node[2 * i + 1].min(node[2 * i + 2]);
            }
        }

        Self { node, n }
    }

    pub fn update(&mut self, i: usize, v: usize) {
        let mut i = i + self.n - 1;

        self.node[i] = self.node[i].min(v);
        while i > 0 {
            i = (i - 1) / 2;
            self.node[i] = self.node[i].min(v);
        }
    }

    pub fn query(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> usize {
        if r <= a || b <= l {
            return std::usize::MAX;
        }
        if a <= l && r <= b {
            self.node[k]
        } else {
            let c1 = self.query(a, b, 2 * k + 1, l, (l + r) / 2);
            let c2 = self.query(a, b, 2 * k + 2, (l + r) / 2, r);
            c1.min(c2)
        }
    }
}

fn main() {
    input! {
        l: usize,
        q: usize,
        cx: [(usize, usize); q]
    }

    let mut values = BTreeSet::new();
    for i in 0..q {
        values.insert(cx[i].1);
    }
    let values = values.into_iter().collect::<Vec<_>>();

    let mut map = HashMap::new();
    for i in 0..values.len() {
        map.insert(values[i], i);
    }

    let mut max_segtree = MaxSegmentTree::new(vec![0; values.len()]);
    let mut min_segtree = MinSegmentTree::new(vec![l; values.len()]);

    for i in 0..q {
        let (c, x) = cx[i];
        let j = map[&x];
        if c == 1 {
            max_segtree.update(j, x);
            min_segtree.update(j, x);
        } else {
            let r = min_segtree.query(j, values.len(), 0, 0, min_segtree.n)
                - max_segtree.query(0, j + 1, 0, 0, max_segtree.n);
            println!("{}", r);
        }
    }
}
