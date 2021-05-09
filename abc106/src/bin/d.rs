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
                node[i] = node[2 * i + 1] + node[2 * i + 2];
            }
        }

        Self { node, n }
    }

    pub fn update(&mut self, i: usize, v: usize) {
        let mut i = i + self.n - 1;

        self.node[i] += v;
        while i > 0 {
            i = (i - 1) / 2;
            self.node[i] += v;
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
            c1 + c2
        }
    }
}
fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        mut lr: [(usize, usize); m],
        pq: [(usize, usize); q]
    }

    lr.sort_unstable_by(|a, b| b.1.cmp(&a.1));
    let mut pq = pq
        .into_iter()
        .enumerate()
        .collect::<Vec<(usize, (usize, usize))>>();
    pq.sort_unstable_by(|a, b| a.1 .1.cmp(&b.1 .1));

    let mut result = vec![0; q];
    let mut segment_tree = SegmentTree::new(vec![0; n]);

    for (i, (p, q)) in pq {
        while !lr.is_empty() && lr.last().unwrap().1 <= q {
            let (l, _) = lr.pop().unwrap();
            segment_tree.update(l - 1, 1);
        }
        let r = segment_tree.query(p - 1, q, 0, 0, segment_tree.n);
        result[i] = r;
    }

    for r in result {
        println!("{}", r);
    }
}
