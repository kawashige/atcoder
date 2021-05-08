use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        x: [i32; n]
    }

    let mut has_zero = false;
    let mut minus = Vec::new();
    let mut plus = Vec::new();
    for xx in x {
        if xx == 0 {
            has_zero = true;
        } else if xx < 0 {
            minus.push(xx.abs());
        } else {
            plus.push(xx);
        }
    }

    let k = if has_zero { k - 1 } else { k };

    if k == 0 {
        println!("0");
        return;
    }

    minus.sort_unstable();

    let mut min = std::i32::MAX;
    for i in 0..plus.len() {
        if i + 1 > k {
            break;
        }
        let mut t = plus[i];
        if i + 1 < k {
            if k - i - 2 < minus.len() {
                t = t * 2 + minus[k - i - 2];
            } else {
                continue;
            }
        }
        min = std::cmp::min(min, t);
    }

    for i in 0..minus.len() {
        if i + 1 > k {
            break;
        }
        let mut t = minus[i];
        if i + 1 < k {
            if k - i - 2 < plus.len() {
                t = t * 2 + plus[k - i - 2];
            } else {
                continue;
            }
        }
        min = std::cmp::min(min, t);
    }

    println!("{}", min);
}
