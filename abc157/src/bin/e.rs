use proconio::input;
use proconio::marker::Chars;
struct SegmentTree {
    node: Vec<i32>,
    pub n: usize,
}

impl SegmentTree {
    pub fn new(nums: Vec<i32>) -> Self {
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

    pub fn update(&mut self, i: usize, v: i32) {
        let mut i = i + self.n - 1;

        self.node[i] += v;
        while i > 0 {
            i = (i - 1) / 2;
            self.node[i] += v;
        }
    }

    pub fn query(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> i32 {
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
        mut s: Chars,
        q: usize,
    }

    let mut counts = vec![vec![0; n]; 26];
    for (i, c) in s.iter().enumerate() {
        counts[*c as usize - 0x61][i] = 1;
    }
    let mut st = counts
        .into_iter()
        .map(|v| SegmentTree::new(v))
        .collect::<Vec<SegmentTree>>();

    for _ in 0..q {
        input! {
            t: char
        }
        if t == '1' {
            input! {
                i: usize,
                c: char,
            }
            st[s[i - 1] as usize - 0x61].update(i - 1, -1);
            st[c as usize - 0x61].update(i - 1, 1);
            s[i - 1] = c;
        } else {
            input! {
                l: usize,
                r: usize
            }
            let count = (0..26)
                .filter(|i| st[*i].query(l - 1, r, 0, 0, st[0].n) > 0)
                .count();
            println!("{}", count);
        }
    }
}
