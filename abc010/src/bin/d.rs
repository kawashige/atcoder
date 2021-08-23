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
        g: usize,
        e: usize,
        p: [usize; g],
        ab: [(usize, usize); e]
    }

    let mut ford_fulkerson = FordFulkerson::new(n + 1);

    for (a, b) in ab {
        ford_fulkerson.add_edge(a, b, 1);
        ford_fulkerson.add_edge(b, a, 1);
    }

    for x in p {
        ford_fulkerson.add_edge(x, n, 1);
    }

    println!("{}", ford_fulkerson.max_flow(0, n));
}
