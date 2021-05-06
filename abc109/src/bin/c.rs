use proconio::input;

fn gcd(a: usize, b: usize) -> usize {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}

fn main() {
    input! {
        n: usize,
        x: usize,
        mut v: [usize; n]
    }

    v.push(x);
    v.sort_unstable();
    let mut m = v[1] - v[0];
    for i in 2..=n {
        m = gcd(m, v[i] - v[i - 1]);
    }

    println!("{}", m);
}
