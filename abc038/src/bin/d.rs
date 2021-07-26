use proconio::input;
struct SegmentTree {
    node: Vec<usize>,
    pub n: usize,
}

impl SegmentTree {
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
                node[i] = node[2 * i + 1].max(node[2 * i + 2]);
            }
        }

        Self { node, n }
    }

    pub fn update(&mut self, i: usize, v: usize) {
        let mut i = i + self.n - 1;

        self.node[i] = self.node[i].max(v);
        while i > 0 {
            i = (i - 1) / 2;
            self.node[i] = self.node[i].max(v);
        }
    }

    pub fn query(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> usize {
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
        n: usize,
        mut wh: [(usize, usize); n]
    }

    let mut segtree = SegmentTree::new(vec![0; 100_001]);

    wh.sort_unstable_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

    let mut i = 0;
    while i < n {
        let mut target = vec![wh[i]];
        while i + 1 < n && wh[i].0 == wh[i + 1].0 {
            i += 1;
            target.push(wh[i]);
        }

        let mut result = Vec::new();
        for (_, h) in target {
            result.push((h, segtree.query(0, h, 0, 0, segtree.n) + 1));
        }

        for (h, c) in result {
            segtree.update(h, c);
        }

        i += 1;
    }

    println!("{}", segtree.query(0, 100001, 0, 0, segtree.n));
}
