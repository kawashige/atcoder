use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut v = Vec::new();
    let mut left = vec![0; n];
    let mut right = vec![0; n];

    for i in 0..n {
        match v.binary_search(&a[i]) {
            Ok(j) => {
                left[i] = j;
            }
            Err(j) => {
                left[i] = j;
                if j < v.len() {
                    v[j] = a[i];
                } else {
                    v.push(a[i]);
                }
            }
        }
    }

    v.clear();
    for i in (0..n).rev() {
        match v.binary_search(&a[i]) {
            Ok(j) => {
                right[i] = j;
            }
            Err(j) => {
                right[i] = j;
                if j < v.len() {
                    v[j] = a[i];
                } else {
                    v.push(a[i]);
                }
            }
        }
    }

    let max = (0..n).map(|i| left[i] + right[i]).max().unwrap();
    println!("{}", max + 1)
}
