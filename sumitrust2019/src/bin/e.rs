use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    const M: usize = 1_000_000_007;

    let mut count = vec![0; n + 1];
    count[0] = 3;

    let mut r = 1;
    for i in 0..n {
        r *= count[a[i]];
        r %= M;
        count[a[i]] -= 1;
        count[a[i] + 1] += 1;
    }

    println!("{}", r);
}
