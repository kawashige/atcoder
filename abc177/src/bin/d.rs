use proconio::input;
use std::collections::HashMap;

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

        if self.size(parent_i) > self.size(parent_j) {
            std::mem::swap(&mut parent_i, &mut parent_j);
        }

        self.parent[parent_j] = parent_i;
        self.size[parent_i] += self.size[parent_j];
    }

    fn size(&mut self, i: usize) -> usize {
        let root = self.root(i);
        self.size[root]
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        friends: [(usize, usize); m]
    }

    let mut disjoint_set = DisjointSet::new(n + 1);
    for (a, b) in friends {
        disjoint_set.unite(a, b);
    }

    let max = *(1..=n)
        .fold(HashMap::new(), |mut count, i| {
            *count.entry(disjoint_set.root(i)).or_insert(0) += 1;
            count
        })
        .values()
        .max()
        .unwrap();
    println!("{}", max);
}
