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
        a: [usize; n]
    }

    let mut set = DisjointSet::new(2 * 100000);
    let mut target = HashSet::new();

    for i in 0..(n / 2) {
        if a[i] != a[n - 1 - i] {
            set.unite(a[i] - 1, a[n - 1 - i] - 1);
            target.insert(a[i] - 1);
            target.insert(a[n - 1 - i] - 1);
        }
    }

    let mut parents = HashSet::new();
    for x in target {
        parents.insert(set.root(x));
    }

    let mut r = 0;
    for p in parents {
        r += set.size(p) - 1;
    }

    println!("{}", r);
}
