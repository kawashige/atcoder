use proconio::input;

fn dfs(
    p: &[usize],
    c: &[i64],
    i: usize,
    k: usize,
    scores: &mut Vec<i64>,
    seen: &mut Vec<bool>,
) -> i64 {
    if seen[i] {
        let l = scores.len();
        let score_sum = scores.iter().sum::<i64>();
        let mut score = std::i64::MIN;
        if score_sum <= 0 || k <= l {
            let max = std::cmp::min(k, l);
            for i in 1..=max {
                let mut tmp = scores.iter().take(i).sum::<i64>();
                score = std::cmp::max(tmp, score);

                for j in 1..l {
                    tmp += scores[(j + i - 1) % l] - scores[(j + l - 1) % l];
                    score = std::cmp::max(tmp, score);
                }
            }
        } else {
            let remains = k % l;
            for i in 0..l {
                let mut max = 0;
                let mut tmp = 0;
                for j in 0..remains {
                    tmp += scores[(i + j) % l];
                    max = std::cmp::max(max, tmp);
                }
                tmp = 0;
                for j in 0..(l - remains) {
                    tmp -= scores[(i + l - j) % l];
                    max = std::cmp::max(max, tmp);
                }
                score = std::cmp::max(score, max + score_sum * (k / l) as i64);
            }
        }

        score
    } else {
        scores.push(c[i]);
        seen[i] = true;
        dfs(p, c, p[i] - 1, k, scores, seen)
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [usize; n],
        c: [i64; n]
    }

    let mut max = std::i64::MIN;
    let mut seen = vec![false; n];

    for i in 0..n {
        if seen[i] {
            continue;
        }
        let r = dfs(&p, &c, i, k, &mut Vec::new(), &mut seen);
        max = std::cmp::max(max, r);
    }

    println!("{}", max);
}
