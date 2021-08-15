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
        mut uvw: [(usize, usize, u64); n - 1]
    }

    uvw.sort_unstable_by(|a, b| a.2.cmp(&b.2));

    let mut disjoint_set = DisjointSet::new(n);
    let mut r = 0;

    for (u, v, w) in uvw {
        r += disjoint_set.size(u - 1) as u64 * disjoint_set.size(v - 1) as u64 * w;
        disjoint_set.unite(u - 1, v - 1);
    }

    println!("{}", r);
}
