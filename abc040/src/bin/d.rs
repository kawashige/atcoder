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
        mut aby: [(usize, usize, usize); m],
        q: usize,
        vw: [(usize, usize); q]
    }

    let mut disjoint_set = DisjointSet::new(n);

    aby.sort_unstable_by(|a, b| a.2.cmp(&b.2));

    let mut vw = vw
        .into_iter()
        .enumerate()
        .collect::<Vec<(usize, (usize, usize))>>();
    vw.sort_unstable_by(|a, b| b.1 .1.cmp(&a.1 .1));

    let mut result = vec![0; q];

    for (i, (v, w)) in vw {
        while !aby.is_empty() && aby[aby.len() - 1].2 > w {
            if let Some((a, b, _)) = aby.pop() {
                disjoint_set.unite(a - 1, b - 1);
            }
        }

        result[i] = disjoint_set.size(v - 1);
    }

    for r in result {
        println!("{}", r);
    }
}
