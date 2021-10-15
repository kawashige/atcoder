use proconio::input;

fn main() {
    input! {
        n: usize,
        mut t: [usize; n]
    }

    const M: usize = 1_000_000_007;

    t.sort_unstable();

    let mut c = 1;
    let mut r = 1;
    let mut p = t[0];
    let mut m = t[0];

    for i in 1..n {
        m += t[i];
        p += m;
        if t[i - 1] == t[i] {
            c += 1;
            r *= c;
            r %= M;
        } else {
            c = 1;
        }
    }

    println!("{}", p);
    println!("{}", r);
}
