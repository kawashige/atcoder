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
        q: usize,
        txyv: [(usize, usize, usize, i64); q]
    }

    let mut sum = vec![0; n];
    for i in 0..q {
        if txyv[i].0 == 0 {
            sum[txyv[i].1 - 1] = txyv[i].3;
        }
    }

    let mut potential = vec![0; n];
    for i in 0..(n - 1) {
        potential[i + 1] = sum[i] - potential[i];
    }

    let mut disjoint_set = DisjointSet::new(n);

    for (t, x, y, v) in txyv {
        if t == 0 {
            disjoint_set.unite(x - 1, y - 1)
        } else {
            if disjoint_set.root(x - 1) != disjoint_set.root(y - 1) {
                println!("Ambiguous");
            } else if (x - 1 + y - 1) % 2 == 0 {
                println!("{}", v + potential[y - 1] - potential[x - 1]);
            } else {
                println!("{}", potential[y - 1] + potential[x - 1] - v);
            }
        }
    }
}
