use proconio::input;
use std::collections::HashMap;
pub struct DisjointSet {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DisjointSet {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    pub fn root(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            return i;
        }

        let parent = self.root(self.parent[i]);
        self.parent[i] = parent;
        parent
    }

    pub fn unite(&mut self, i: usize, j: usize) {
        let parent_i = self.root(i);
        let parent_j = self.root(j);

        if parent_i == parent_j {
            return;
        }

        self.parent[parent_j] = parent_i;
        self.size[parent_i] += self.size[parent_j];
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        c: [usize; n],
        query: [[usize; 3]; q]
    }

    let mut counts = HashMap::new();
    for i in 0..n {
        counts.insert(
            i,
            vec![(c[i], 1)]
                .into_iter()
                .collect::<HashMap<usize, usize>>(),
        );
    }

    let mut disjoint_set = DisjointSet::new(n);

    for v in query {
        if v[0] == 1 {
            let parent_a = disjoint_set.root(v[1] - 1);
            let parent_b = disjoint_set.root(v[2] - 1);
            if parent_a == parent_b {
                continue;
            }

            if counts.get(&parent_a).unwrap().len() > counts.get(&parent_b).unwrap().len() {
                disjoint_set.unite(v[1] - 1, v[2] - 1);

                let b_counts = counts.remove(&parent_b).unwrap();
                let a_counts = counts.get_mut(&parent_a).unwrap();
                for (k, v) in b_counts {
                    *a_counts.entry(k).or_insert(0) += v;
                }
            } else {
                disjoint_set.unite(v[2] - 1, v[1] - 1);

                let a_counts = counts.remove(&parent_a).unwrap();
                let b_counts = counts.get_mut(&parent_b).unwrap();
                for (k, v) in a_counts {
                    *b_counts.entry(k).or_insert(0) += v;
                }
            }
        } else {
            let count = counts
                .get(&disjoint_set.root(v[1] - 1))
                .unwrap()
                .get(&(v[2]))
                .unwrap_or(&0);
            println!("{}", count);
        }
    }
}
