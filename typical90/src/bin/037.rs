use proconio::input;
struct SegmentTree {
    node: Vec<u64>,
    pub n: usize,
}

impl SegmentTree {
    pub fn new(nums: Vec<u64>) -> Self {
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
                node[i] = node[2 * i + 1].max(node[2 * i + 2]);
            }
        }

        Self { node, n }
    }

    pub fn update(&mut self, i: usize, v: u64) {
        let mut i = i + self.n - 1;

        self.node[i] = self.node[i].max(v);
        while i > 0 {
            i = (i - 1) / 2;
            self.node[i] = self.node[i].max(v);
        }
    }

    pub fn query(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> u64 {
        if r <= a || b <= l {
            return 0;
        }
        if a <= l && r <= b {
            self.node[k]
        } else {
            let c1 = self.query(a, b, 2 * k + 1, l, (l + r) / 2);
            let c2 = self.query(a, b, 2 * k + 2, (l + r) / 2, r);
            c1.max(c2)
        }
    }
}
fn main() {
    input! {
        w: usize,
        n: usize,
        lrv: [(usize, usize, u64); n]
    }

    let mut segtree = SegmentTree::new(vec![0; w + 1]);

    for (l, r, v) in lrv {
        for i in (l..=w).rev() {
            let max = segtree.query(if i < r { 0 } else { i - r }, i - l + 1, 0, 0, segtree.n);
            if max > 0 || i <= r {
                segtree.update(i, max + v);
            }
        }
    }

    let r = segtree.query(w, w + 1, 0, 0, segtree.n);
    if r == 0 {
        println!("-1");
    } else {
        println!("{}", r);
    }
}
