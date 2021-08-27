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
        mut clr: [(u64, usize, usize); m]
    }
    clr.sort_unstable();

    let mut ds = DisjointSet::new(n + 1);
    let mut min = 0;
    for (c, l, r) in clr {
        if ds.root(l - 1) != ds.root(r) {
            ds.unite(l - 1, r);
            min += c;
        }
    }

    if ds.size(0) < n + 1 {
        println!("-1");
    } else {
        println!("{}", min);
    }
}
