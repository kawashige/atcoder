use proconio::input;
struct LazySegmentTree {
    node: Vec<usize>,
    lazy: Vec<usize>,
    pub n: usize,
}

impl LazySegmentTree {
    pub fn new(nums: Vec<usize>) -> Self {
        let size = nums.len();

        let mut n = 1;
        while n < size {
            n *= 2;
        }
        let mut node = vec![0; 2 * n - 1];

        for i in 0..size {
            node[i + n - 1] = nums[i];
        }
        if n > 2 {
            for i in (0..=(n - 2)).rev() {
                node[i] = node[2 * i + 1].min(node[2 * i + 2]);
            }
        }

        Self {
            node,
            n,
            lazy: vec![std::usize::MAX; 2 * n - 1],
        }
    }

    fn eval(&mut self, k: usize) {
        if self.lazy[k] == std::usize::MAX {
            return;
        }
        if k < self.n - 1 {
            self.lazy[k * 2 + 1] = self.lazy[k * 2 + 1].min(self.lazy[k]);
            self.lazy[k * 2 + 2] = self.lazy[k * 2 + 2].min(self.lazy[k]);
        }

        self.node[k] = self.node[k].min(self.lazy[k]);
        self.lazy[k] = std::usize::MAX;
    }

    pub fn update_range(&mut self, a: usize, b: usize, x: usize, k: usize, l: usize, r: usize) {
        self.eval(k);
        if a <= l && r <= b {
            self.lazy[k] = self.lazy[k].min(x);
            self.eval(k);
        } else if a < r && l < b {
            self.update_range(a, b, x, 2 * k + 1, l, (l + r) / 2);
            self.update_range(a, b, x, 2 * k + 2, (l + r) / 2, r);

            self.node[k] = self.node[k]
                .min(self.node[2 * k + 1])
                .min(self.node[2 * k + 2]);
        }
    }

    pub fn update(&mut self, a: usize, b: usize, x: usize) {
        self.update_range(a, b, x, 0, 0, self.n)
    }

    pub fn query_range(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> usize {
        self.eval(k);
        if r <= a || b <= l {
            std::usize::MAX
        } else if a <= l && r <= b {
            self.node[k]
        } else {
            let c1 = self.query_range(a, b, 2 * k + 1, l, (l + r) / 2);
            let c2 = self.query_range(a, b, 2 * k + 2, (l + r) / 2, r);
            c1.min(c2)
        }
    }

    pub fn query(&mut self, a: usize, b: usize) -> usize {
        self.query_range(a, b, 0, 0, self.n)
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        queries: [(usize, usize); q]
    }

    let mut column = LazySegmentTree::new(vec![n - 2; n - 2]);
    let mut row = LazySegmentTree::new(vec![n - 2; n - 2]);

    let mut r = (n - 2) as u64 * (n - 2) as u64;

    for (t, x) in queries {
        if t == 1 {
            let c = column.query(x - 2, x - 1);
            r -= c as u64;
            row.update(0, c, x - 2);
        } else {
            let c = row.query(x - 2, x - 1);
            r -= c as u64;
            column.update(0, c, x - 2);
        }
    }

    println!("{}", r);
}
