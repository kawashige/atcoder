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
        let mut node = vec![std::usize::MAX; 2 * n - 1];

        for i in 0..size {
            node[i + n - 1] = nums[i];
        }
        if n > 2 {
            for i in (0..=(n - 2)).rev() {
                node[i] = node[2 * i + 1].min(node[2 * i + 2]);
            }
        }

        Self { node, n }
    }

    pub fn update(&mut self, i: usize, v: usize) {
        let mut i = i + self.n - 1;

        self.node[i] = v;
        while i > 0 {
            i = (i - 1) / 2;
            self.node[i] = self.node[i * 2 + 1].min(self.node[i * 2 + 2]);
        }
    }

    pub fn query(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> usize {
        if r <= a || b <= l {
            return std::usize::MAX;
        }
        if a <= l && r <= b {
            self.node[k]
        } else {
            let c1 = self.query(a, b, 2 * k + 1, l, (l + r) / 2);
            let c2 = self.query(a, b, 2 * k + 2, (l + r) / 2, r);
            c1.min(c2)
        }
    }
}
fn main() {
    input! {
        q: usize,
        tx: [(usize, i64); q]
    }

    const N: usize = 1048576;

    let mut segtree = SegmentTree::new((0..N).collect::<Vec<_>>());
    let mut nums = vec![-1; N];

    for (t, x) in tx {
        if t == 1 {
            let mut i = segtree.query((x % N as i64) as usize, N, 0, 0, segtree.n);

            if i == N {
                i = segtree.query(0, (x % N as i64) as usize, 0, 0, segtree.n);
            }
            nums[i] = x;
            segtree.update(i, N);
        } else {
            println!("{}", nums[(x % N as i64) as usize]);
        }
    }
}
