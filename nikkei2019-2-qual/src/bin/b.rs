use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [usize; n]
    }
    const M: usize = 998244353;

    let mut count = vec![0; d.iter().max().unwrap() + 1];
    for i in 0..n {
        count[d[i]] += 1;
    }

    if d[0] != 0 || count[0] != 1 {
        println!("0");
        return;
    }

    let mut r = 1;
    for i in 1..count.len() {
        for _ in 0..count[i] {
            r *= count[i - 1];
            r %= M;
        }
    }

    println!("{}", r);
}
