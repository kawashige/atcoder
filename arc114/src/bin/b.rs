use std::collections::HashSet;

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
        f: [usize; n]
    }

    let mut ds = DisjointSet::new(n);

    for i in 0..n {
        ds.unite(i, f[i] - 1);
    }

    let mut parents = HashSet::new();
    for i in 0..n {
        parents.insert(ds.root(i));
    }

    let ans = (0..parents.len()).fold(1_usize, |acc, _| acc * 2 % 998244353) - 1;

    println!("{}", ans);
}
