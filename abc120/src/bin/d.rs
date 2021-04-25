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

        if self.size(parent_i) > self.size(parent_j) {
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
        m: usize,
        ab: [(usize, usize); m]
    }

    let mut disjoint_set = DisjointSet::new(n);
    let mut result = Vec::new();

    for i in (0..m).rev() {
        if disjoint_set.root(ab[i].0 - 1) == disjoint_set.root(ab[i].1 - 1) {
            result.push(0);
        } else {
            result.push(disjoint_set.size(ab[i].0 - 1) * disjoint_set.size(ab[i].1 - 1));
            disjoint_set.unite(ab[i].0 - 1, ab[i].1 - 1);
        }
    }

    let mut count: u64 = 0;
    for r in result.into_iter().rev() {
        count += r as u64;
        println!("{}", count);
    }
}
