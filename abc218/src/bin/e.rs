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
        m: usize,
        mut abc: [(usize, usize, i64); m]
    }

    abc.sort_unstable_by_key(|(_, _, c)| *c);

    let mut ds = DisjointSet::new(n);
    let mut r = 0;

    for (a, b, c) in abc {
        if ds.root(a - 1) != ds.root(b - 1) {
            ds.unite(a - 1, b - 1);
        } else if c > 0 {
            r += c;
        }
    }

    println!("{}", r);
}
