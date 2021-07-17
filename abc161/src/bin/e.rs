use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        k: usize,
        c: usize,
        s: Chars
    }

    let mut count = vec![0; n];
    let mut max = vec![-1; n];

    for i in 0..n {
        if s[i] == 'x' {
            count[i] = -1;
        } else {
            if i < c + 1 || max[i - c - 1] == -1 {
                count[i] = 1;
            } else {
                count[i] = max[i - c - 1] + 1;
            }
        }
        if i == 0 {
            max[i] = count[i];
        } else {
            max[i] = max[i - 1].max(count[i]);
        }
    }

    if max[n - 1] == k as i32 {
        let mut results = Vec::new();

        let mut target = k as i32;
        let mut indices = Vec::new();
        for i in (0..n).rev() {
            if max[i] != target {
                break;
            }

            if count[i] == target {
                indices.push(i);
            }
        }

        if indices.len() == 1 {
            results.push(indices[0]);
        }

        let mut e = indices[0].saturating_sub(c + 1);
        target -= 1;

        while target > 0 {
            let mut indices = Vec::new();
            for i in (0..=e).rev() {
                if max[i] < target {
                    break;
                }
                if count[i] == target {
                    indices.push(i);
                }
            }

            if indices.len() == 1 {
                results.push(indices[0]);
            }

            e = indices[0].saturating_sub(c + 1);
            target -= 1;
        }

        for r in results.into_iter().rev() {
            println!("{}", r + 1);
        }
    }
}
