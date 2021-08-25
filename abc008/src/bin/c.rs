use proconio::input;

fn main() {
    input! {
        n: usize,
        mut c: [usize; n]
    }

    let mut count = vec![0; n];
    for i in 0..n {
        for j in 0..n {
            if c[i] % c[j] == 0 {
                count[i] += 1;
            }
        }
    }

    let mut r = 0.0;
    for c in count {
        r += (((c - 1) / 2) + 1) as f64 / c as f64;
    }

    println!("{}", r);
}
