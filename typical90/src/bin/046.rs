use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
    }

    let mut r_a = vec![0; 46];
    let mut r_b = vec![0; 46];
    let mut r_c = vec![0; 46];

    for i in 0..n {
        r_a[a[i] % 46] += 1;
        r_b[b[i] % 46] += 1;
        r_c[c[i] % 46] += 1;
    }

    let mut count: u64 = 0;
    for i in 0..46 {
        for j in 0..46 {
            for k in 0..46 {
                if (i + j + k) % 46 == 0 {
                    count += r_a[i] * r_b[j] * r_c[k];
                }
            }
        }
    }

    println!("{}", count);
}
