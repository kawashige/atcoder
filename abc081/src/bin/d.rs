use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i32; n]
    }

    let min_i = (0..n).min_by_key(|i| a[*i]).unwrap();
    let max_i = (0..n).max_by_key(|i| a[*i]).unwrap();

    let mut results = Vec::new();

    if a[min_i] < 0 && 0 < a[max_i] {
        if a[min_i].abs() < a[max_i] {
            for i in 0..n {
                if i != max_i && a[i] < 0 {
                    a[i] += a[max_i];
                    results.push((max_i, i));
                }
            }
            let mut target = max_i;
            for i in 0..n {
                a[i] += a[target];
                results.push((target, i));
                target = i;
            }
        } else {
            for i in 0..n {
                if i != min_i && a[i] > 0 {
                    a[i] += a[min_i];
                    results.push((min_i, i));
                }
            }
            let mut target = min_i;
            for i in (0..n).rev() {
                a[i] += a[target];
                results.push((target, i));
                target = i;
            }
        };
    } else if a[max_i] > 0 {
        let mut target = max_i;
        for i in 0..n {
            a[i] += a[target];
            results.push((target, i));
            target = i;
        }
    } else if a[min_i] < 0 {
        let mut target = min_i;
        for i in (0..n).rev() {
            a[i] += a[target];
            results.push((target, i));
            target = i;
        }
    }

    println!("{}", results.len());
    for (i, j) in results {
        println!("{} {}", i + 1, j + 1);
    }
}
