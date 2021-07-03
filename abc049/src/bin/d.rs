use std::collections::HashMap;

use proconio::input;
pub struct DisjointSet {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DisjointSet {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    pub fn root(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            return i;
        }

        let parent = self.root(self.parent[i]);
        self.parent[i] = parent;
        parent
    }

    pub fn unite(&mut self, i: usize, j: usize) {
        let mut parent_i = self.root(i);
        let mut parent_j = self.root(j);

        if parent_i == parent_j {
            return;
        }

        if self.size(parent_i) < self.size(parent_j) {
            std::mem::swap(&mut parent_i, &mut parent_j);
        }

        self.parent[parent_j] = parent_i;
        self.size[parent_i] += self.size[parent_j];
    }

    pub fn size(&mut self, i: usize) -> usize {
        let root = self.root(i);
        self.size[root]
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
        l: usize,
        pq: [(usize, usize); k],
        rs: [(usize, usize); l],
    }

    let mut ds1 = DisjointSet::new(n);

    for (p, q) in pq {
        ds1.unite(p - 1, q - 1)
    }

    let mut ds2 = DisjointSet::new(n);

    for (r, s) in rs {
        ds2.unite(r - 1, s - 1);
    }

    let mut count = HashMap::new();
    for i in 0..n {
        *count.entry((ds1.root(i), ds2.root(i))).or_insert(0) += 1;
    }

    for i in 0..n {
        println!("{}", count[&(ds1.root(i), ds2.root(i))]);
    }
}
