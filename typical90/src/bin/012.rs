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
        h: usize,
        w: usize,
        q_n: usize,
    }

    let mut st = DisjointSet::new(h * w);
    let mut board = vec![vec![false; w]; h];

    for _ in 0..q_n {
        input! {
            t: usize
        }
        if t == 1 {
            input! {
                r: usize,
                c: usize
            }
            let r = r - 1;
            let c = c - 1;

            board[r][c] = true;

            let i = r * w + c;
            if r > 0 && board[r - 1][c] {
                st.unite(i, (r - 1) * w + c);
            }
            if c > 0 && board[r][c - 1] {
                st.unite(i, r * w + c - 1);
            }
            if r < h - 1 && board[r + 1][c] {
                st.unite(i, (r + 1) * w + c);
            }
            if c < w - 1 && board[r][c + 1] {
                st.unite(i, r * w + c + 1);
            }
        } else {
            input! {
                r1: usize,
                c1: usize,
                r2: usize,
                c2: usize,
            }
            let r1 = r1 - 1;
            let c1 = c1 - 1;
            let r2 = r2 - 1;
            let c2 = c2 - 1;

            let i1 = r1 * w + c1;
            let i2 = r2 * w + c2;

            if board[r1][c1] && board[r2][c2] && st.root(i1) == st.root(i2) {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
