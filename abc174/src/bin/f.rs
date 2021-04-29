use proconio::fastout;
use proconio::input;
struct SegmentTree {
    node: Vec<Vec<usize>>,
    pub n: usize,
}

impl SegmentTree {
    pub fn new(nums: Vec<usize>) -> Self {
        let size = nums.len();

        let mut n = 1;
        while n < size {
            n *= 2;
        }
        let mut node = vec![vec![]; 2 * n - 1];

        for i in 0..size {
            node[i + n - 1] = vec![nums[i]];
        }
        if n > 2 {
            for i in (0..=(n - 2)).rev() {
                let mut j = 0;
                let mut k = 0;
                let mut v = Vec::with_capacity(node[2 * i + 1].len() + node[2 * i + 2].len());
                while j < node[2 * i + 1].len() || k < node[2 * i + 2].len() {
                    if j < node[2 * i + 1].len()
                        && (k >= node[2 * i + 2].len() || node[2 * i + 1][j] <= node[2 * i + 2][k])
                    {
                        v.push(node[2 * i + 1][j]);
                        j += 1;
                    }
                    if k < node[2 * i + 2].len()
                        && (j >= node[2 * i + 1].len() || node[2 * i + 1][j] > node[2 * i + 2][k])
                    {
                        v.push(node[2 * i + 2][k]);
                        k += 1;
                    }
                }
                node[i] = v;
            }
        }

        Self { node, n }
    }

    pub fn query(&self, a: usize, b: usize, k: usize, l: usize, r: usize, x: usize) -> usize {
        if r <= a || b <= l {
            return 0;
        }
        if a <= l && r <= b {
            self.node[k].binary_search(&x).map_or_else(|e| e, |i| i)
        } else {
            let c1 = self.query(a, b, 2 * k + 1, l, (l + r) / 2, x);
            let c2 = self.query(a, b, 2 * k + 2, (l + r) / 2, r, x);
            c1 + c2
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        c: [usize; n],
        lr: [(usize, usize); q]
    }

    let mut nums = vec![-1_i32; n + 1];
    let v = c
        .iter()
        .enumerate()
        .map(|(i, x)| {
            let tmp = (nums[*x] + 1) as usize;
            nums[*x] = i as i32;
            tmp
        })
        .collect::<Vec<usize>>();

    let st = SegmentTree::new(v);

    for (l, r) in lr {
        let result = st.query(l - 1, r, 0, 0, st.n, l);
        println!("{}", result);
    }
}
