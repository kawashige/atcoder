use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        lr: [(usize, usize); q]
    }

    let mut flip = vec![0; n + 1];
    for (l, r) in lr {
        flip[l - 1] += 1;
        flip[r] -= 1;
    }

    let mut r = Vec::with_capacity(n);
    let mut acc = 0;
    for i in 0..n {
        acc += flip[i];
        r.push(if acc % 2 == 0 { '0' } else { '1' });
    }

    println!("{}", r.into_iter().collect::<String>());
}
