use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [[usize; 3]; n]
    }

    let mut rank = p
        .into_iter()
        .enumerate()
        .map(|(i, p)| (p.iter().sum::<usize>(), i))
        .collect::<Vec<_>>();
    rank.sort_unstable_by(|a, b| b.0.cmp(&a.0));

    let mut border = 0;
    let mut i = 0;
    while i < n {
        border = rank[i].0;
        while i + 1 < n && rank[i + 1].1 == border {
            i += 1;
        }

        if k - 1 <= i {
            break;
        }

        i += 1;
    }

    let mut r = vec![false; n];

    for (p, i) in rank {
        if border <= p + 300 {
            r[i] = true;
        }
    }

    for b in r {
        println!("{}", if b { "Yes" } else { "No" });
    }
}
