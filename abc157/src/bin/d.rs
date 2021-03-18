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
        k: usize,
        frieds: [(usize, usize); m],
        blocks: [(usize, usize); k],
    }

    let mut disjoint_set = DisjointSet::new(n + 1);
    let mut counts = vec![0; n + 1];

    for (a, b) in frieds {
        disjoint_set.unite(a, b);
        counts[a] += 1;
        counts[b] += 1;
    }

    for i in 1..=n {
        counts[i] = disjoint_set.size(i) - 1 - counts[i];
    }

    for (a, b) in blocks {
        if disjoint_set.root(a) == disjoint_set.root(b) {
            counts[a] -= 1;
            counts[b] -= 1;
        }
    }

    for i in 1..=n {
        if i != 1 {
            print!(" ");
        }
        print!("{}", counts[i]);
    }
}
