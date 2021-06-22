use proconio::input;

fn main() {
    input! {
        n: usize,
        w: u64,
        wv: [(usize, usize); n]
    }

    let mut weight = vec![vec![]; 4];
    let base = wv[0].0;

    for (w, v) in wv {
        weight[w - base].push(v);
    }

    for i in 0..weight.len() {
        weight[i].sort_unstable_by(|a, b| b.cmp(&a));
        weight[i].insert(0, 0);
        for j in 2..weight[i].len() {
            weight[i][j] += weight[i][j - 1];
        }
    }

    let mut max = 0;
    for i in 0..weight[0].len() {
        let sum = (base * i) as u64;
        if sum > w {
            break;
        }
        for j in 0..weight[1].len() {
            let sum2 = sum + ((base + 1) * j) as u64;
            if sum2 > w {
                break;
            }
            for k in 0..weight[2].len() {
                let sum3 = sum2 + ((base + 2) * k) as u64;
                if sum3 > w {
                    break;
                }
                for l in 0..weight[3].len() {
                    let sum4 = sum3 + ((base + 3) * l) as u64;
                    if sum4 > w {
                        break;
                    }
                    max = std::cmp::max(
                        max,
                        weight[0][i] + weight[1][j] + weight[2][k] + weight[3][l],
                    );
                }
            }
        }
    }

    println!("{}", max);
}
