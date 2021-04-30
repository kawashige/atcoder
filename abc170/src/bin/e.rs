use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        q: usize,
        ab: [(usize, usize); n],
        cd: [(usize, usize); q]
    }

    let c = 2 * 100_000 + 1;

    let mut children = vec![(0, 0); n + 1];
    let mut yochien = vec![BTreeMap::new(); c];
    let mut equality = BTreeMap::new();

    for i in 0..n {
        children[i + 1] = (ab[i].0, ab[i].1);
        *yochien[ab[i].1].entry(ab[i].0).or_insert(0) += 1;
    }
    for i in 1..c {
        if !yochien[i].is_empty() {
            let min = *yochien[i].iter().next_back().unwrap().0;
            *equality.entry(min).or_insert(0) += 1;
        }
    }

    for (c, d) in cd {
        let count = *yochien[children[c].1].get(&children[c].0).unwrap();
        let max = *yochien[children[c].1].iter().next_back().unwrap().0;
        if count > 1 {
            *yochien[children[c].1].get_mut(&children[c].0).unwrap() -= 1;
        } else {
            yochien[children[c].1].remove(&children[c].0);
            if max == children[c].0 {
                let e_count = *equality.get(&max).unwrap();
                if e_count > 1 {
                    *equality.get_mut(&max).unwrap() -= 1;
                } else {
                    equality.remove(&max);
                }
                if let Some(new_max) = yochien[children[c].1].iter().next_back() {
                    *equality.entry(*new_max.0).or_insert(0) += 1;
                };
            }
        }

        if yochien[d].is_empty() {
            *yochien[d].entry(children[c].0).or_insert(0) += 1;
            *equality.entry(children[c].0).or_insert(0) += 1;
        } else {
            let max = *yochien[d].iter().next_back().unwrap().0;
            *yochien[d].entry(children[c].0).or_insert(0) += 1;
            if max < children[c].0 {
                let e_count = *equality.get(&max).unwrap();
                if e_count > 1 {
                    *equality.get_mut(&max).unwrap() -= 1;
                } else {
                    equality.remove(&max);
                }
                *equality.entry(children[c].0).or_insert(0) += 1;
            }
        }

        children[c].1 = d;

        println!("{}", equality.iter().next().unwrap().0);
    }
}
