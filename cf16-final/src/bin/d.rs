use proconio::input;

#[derive(Clone, Copy, Eq, PartialEq)]
struct Edge {
    to: usize,
    cap: i64,
    rev: usize,
}

struct FordFulkerson {
    g: Vec<Vec<Edge>>,
    used: Vec<bool>,
}

impl FordFulkerson {
    fn new(n: usize) -> Self {
        Self {
            g: vec![vec![]; n],
            used: vec![false; n],
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, cap: i64) {
        let to_edge = Edge {
            to,
            cap,
            rev: self.g[to].len(),
        };
        self.g[from].push(to_edge);

        let from_edge = Edge {
            to: from,
            cap: 0,
            rev: self.g[from].len() - 1,
        };
        self.g[to].push(from_edge);
    }

    fn dfs(&mut self, v: usize, t: usize, f: i64) -> i64 {
        if v == t {
            return f;
        }

        self.used[v] = true;

        for i in 0..self.g[v].len() {
            let edge = self.g[v][i];
            if !self.used[edge.to] && 0 < edge.cap {
                let d = self.dfs(edge.to, t, f.min(edge.cap));
                if 0 < d {
                    self.g[v][i].cap -= d;
                    self.g[edge.to][edge.rev].cap += d;
                    return d;
                }
            }
        }
        0
    }

    fn max_flow(&mut self, s: usize, t: usize) -> i64 {
        let mut flow = 0;
        loop {
            self.used = vec![false; self.used.len()];
            let f = self.dfs(s, t, std::i64::MAX);
            if f == 0 {
                return flow;
            }
            flow += f;
        }
    }
}
fn main() {
    input! {
        n: usize,
        m: usize,
        x: [usize; n]
    }

    let mut ford_fulkerson = FordFulkerson::new(2 * n + 2);

    for i in 0..n {
        ford_fulkerson.add_edge(0, i + 1, 1);
        ford_fulkerson.add_edge(i + 1 + n, 2 * n + 1, 1);
    }

    let mut val = vec![vec![]; 100_000];
    let mut rem = vec![vec![]; m];

    for i in 0..n {
        val[x[i]].push(i);
        rem[x[i] % m].push(i);
    }

    for i in 0..n {
        for j in &val[x[i]] {
            if j != &i {
                ford_fulkerson.add_edge(i + 1, j + 1 + n, 1);
            }
        }
        for j in &rem[if x[i] % m == 0 { 0 } else { m - (x[i] % m) }] {
            if j != &i {
                ford_fulkerson.add_edge(i + 1, j + 1 + n, 1);
            }
        }
    }

    let r = ford_fulkerson.max_flow(0, 2 * n + 1);

    println!("{}", r);
}
