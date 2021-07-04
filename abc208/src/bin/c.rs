use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u64,
        a: [usize; n]
    }

    let mut a = a.into_iter().enumerate().collect::<Vec<(usize, usize)>>();
    a.sort_unstable_by(|a, b| a.1.cmp(&b.1));

    let r = k % (n as u64);
    let c = k / (n as u64);

    let mut result = Vec::with_capacity(n);

    for i in 0..n {
        if (i + 1) as u64 <= r {
            result.push((a[i].0, c + 1))
        } else {
            result.push((a[i].0, c))
        }
    }

    result.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    for x in result {
        println!("{}", x.1);
    }
}
