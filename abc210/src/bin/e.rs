use proconio::input;

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
fn main() {
    input! {
        n: usize,
        m: usize,
        mut ac: [(usize, usize); m]
    }

    ac.sort_unstable_by(|a, b| a.1.cmp(&b.1));

    let mut r = 0;
    let mut x = n;
    for i in 0..m {
        let tmp = gcd(x, ac[i].0);
        r += (x - tmp) * ac[i].1;

        if i == m - 1 && tmp > 1 {
            println!("-1");
            return;
        }

        x = tmp;
    }

    println!("{}", r)
}
