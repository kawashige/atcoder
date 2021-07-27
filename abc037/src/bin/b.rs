use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        lrt: [(usize, usize, usize); q]
    }

    let mut a = vec![0; n];
    for (l, r, t) in lrt {
        for i in (l - 1)..r {
            a[i] = t
        }
    }

    for x in a {
        println!("{}", x);
    }
}
