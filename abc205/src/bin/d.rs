use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u64; n],
        k: [u64; q]
    }

    let mut d = vec![0; n];
    let mut index = vec![0; n];
    d[0] = a[0] - 1;
    for i in 1..n {
        d[i] = d[i - 1] + a[i] - a[i - 1] - 1;
        if a[i] == a[i - 1] + 1 {
            index[i] = index[i - 1];
        } else {
            index[i] = i;
        }
    }

    for q in k {
        let x = match d.binary_search(&q) {
            Ok(i) => a[index[i]] - 1,
            Err(i) => {
                if i == n {
                    a[n - 1] + q - d[n - 1]
                } else {
                    a[i] - 1 - (d[i] - q)
                }
            }
        };
        println!("{}", x);
    }
}
