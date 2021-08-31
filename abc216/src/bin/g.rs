use proconio::input;

struct LazySegmentTree {
    pub node: Vec<usize>,
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
                node[i] = node[2 * i + 1] + node[2 * i + 2];
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
            self.lazy[k * 2 + 1] = self.lazy[k];
            self.lazy[k * 2 + 2] = self.lazy[k];
        }

        self.node[k] = self.lazy[k];
        self.lazy[k] = std::usize::MAX;
    }

    pub fn update_range(&mut self, a: usize, b: usize, x: usize, k: usize, l: usize, r: usize) {
        self.eval(k);
        if a <= l && r <= b {
            self.lazy[k] = x;
            self.eval(k);
        } else if a < r && l < b {
            self.update_range(a, b, x, 2 * k + 1, l, (l + r) / 2);
            self.update_range(a, b, x, 2 * k + 2, (l + r) / 2, r);

            self.node[k] = self.node[2 * k + 1] + self.node[2 * k + 2];
        }
    }

    pub fn update(&mut self, a: usize, b: usize, x: usize) {
        self.update_range(a, b, x, 0, 0, self.n)
    }

    pub fn query_range(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> usize {
        self.eval(k);
        if r <= a || b <= l {
            0
        } else if a <= l && r <= b {
            self.node[k]
        } else {
            let c1 = self.query_range(a, b, 2 * k + 1, l, (l + r) / 2);
            let c2 = self.query_range(a, b, 2 * k + 2, (l + r) / 2, r);
            c1 + c2
        }
    }

    pub fn query(&mut self, a: usize, b: usize) -> usize {
        self.query_range(a, b, 0, 0, self.n)
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        mut lrx: [(usize, usize, usize); m]
    }

    lrx.sort_unstable_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.1)));

    let mut segtree = LazySegmentTree::new(vec![0; n]);

    for (l, r, x) in lrx {
        if segtree.query(l - 1, r) >= x {
            continue;
        }

        let mut ok = l - 1;
        let mut ng = r;

        while ok + 1 < ng {
            let mid = (ok + ng) / 2;
            if segtree.query(l - 1, mid) + r - mid >= x {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        segtree.update(ok, r, 1);
    }

    for i in 0..n {
        if i > 0 {
            print!(" ");
        }
        print!("{}", segtree.query(i, i + 1));
    }
    print!("\n");
}
