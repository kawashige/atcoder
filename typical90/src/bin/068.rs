use proconio::input;

struct SegmentTree {
    pub node: Vec<i64>,
    pub n: usize,
}

impl SegmentTree {
    pub fn new(nums: Vec<i64>) -> Self {
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

        Self { node, n }
    }

    pub fn update(&mut self, i: usize, v: i64) {
        let mut i = i + self.n - 1;

        self.node[i] = self.node[i] + v;
        while i > 0 {
            i = (i - 1) / 2;
            self.node[i] = self.node[i] + v;
        }
    }

    pub fn query(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> i64 {
        if r <= a || b <= l {
            return 0;
        }
        if a <= l && r <= b {
            self.node[k]
        } else {
            let c1 = self.query(a, b, 2 * k + 1, l, (l + r) / 2);
            let c2 = self.query(a, b, 2 * k + 2, (l + r) / 2, r);
            c1 + c2
        }
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        txyv: [(usize, usize, usize, i64); q]
    }

    let mut segtree_even = SegmentTree::new(vec![0; n]);
    let mut segtree_odd = SegmentTree::new(vec![0; n]);
    let mut segtree_count = SegmentTree::new(vec![0; n]);

    for (t, x, y, v) in txyv {
        if t == 0 {
            if x % 2 == 0 {
                segtree_even.update(x, v);
            } else {
                segtree_odd.update(x, v);
            }
            segtree_count.update(x, 1);
        } else {
            if x == y {
                println!("{}", v);
            } else if x < y {
                if segtree_count.query(x, y, 0, 0, segtree_count.n) != (y - x) as i64 {
                    println!("Ambiguous");
                } else {
                    let sum = if (y - x) % 2 == 0 { v } else { -v }
                        + if (y - 1) % 2 == 0 {
                            segtree_even.query(x, y, 0, 0, segtree_even.n)
                                - segtree_odd.query(x, y, 0, 0, segtree_even.n)
                        } else {
                            -segtree_even.query(x, y, 0, 0, segtree_even.n)
                                + segtree_odd.query(x, y, 0, 0, segtree_even.n)
                        };
                    println!("{}", sum);
                }
            } else {
                if segtree_count.query(y, x, 0, 0, segtree_count.n) != (x - y) as i64 {
                    println!("Ambiguous");
                } else {
                    let sum = if (x - y) % 2 == 0 { v } else { -v }
                        + if y % 2 == 0 {
                            segtree_even.query(y, x, 0, 0, segtree_even.n)
                                - segtree_odd.query(y, x, 0, 0, segtree_even.n)
                        } else {
                            -segtree_even.query(y, x, 0, 0, segtree_even.n)
                                + segtree_odd.query(y, x, 0, 0, segtree_even.n)
                        };
                    println!("{}", sum);
                }
            }
        }
    }
}
