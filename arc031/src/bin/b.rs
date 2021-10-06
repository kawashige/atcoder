use proconio::input;
use proconio::marker::Chars;

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
        a: [Chars; 10]
    }

    let mut ds = DisjointSet::new(100);
    let mut count = 0;

    for i in 0..10 {
        for j in 0..10 {
            if a[i][j] == 'o' {
                count += 1;
                if i > 0 && a[i - 1][j] == 'o' {
                    ds.unite((i - 1) * 10 + j, i * 10 + j);
                }
                if j > 0 && a[i][j - 1] == 'o' {
                    ds.unite(i * 10 + j - 1, i * 10 + j);
                }
            }
        }
    }

    for i in 0..10 {
        for j in 0..10 {
            if a[i][j] == 'x' {
                let mut tmp = 0;
                let mut seen = Vec::new();
                if i > 0 && a[i - 1][j] == 'o' {
                    tmp += ds.size((i - 1) * 10 + j);
                    seen.push(ds.root((i - 1) * 10 + j));
                }
                if j > 0 && a[i][j - 1] == 'o' && !seen.contains(&ds.root(i * 10 + j - 1)) {
                    tmp += ds.size(i * 10 + j - 1);
                    seen.push(ds.root(i * 10 + j - 1));
                }
                if i < 9 && a[i + 1][j] == 'o' && !seen.contains(&ds.root((i + 1) * 10 + j)) {
                    tmp += ds.size((i + 1) * 10 + j);
                    seen.push(ds.root((i + 1) * 10 + j));
                }
                if j < 9 && a[i][j + 1] == 'o' && !seen.contains(&ds.root(i * 10 + j + 1)) {
                    tmp += ds.size(i * 10 + j + 1);
                }

                if tmp == count {
                    println!("YES");
                    return;
                }
            }
        }
    }

    println!("NO");
}
