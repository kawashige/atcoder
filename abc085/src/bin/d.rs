use proconio::input;

fn main() {
    input! {
        n: usize,
        h: usize,
        ab: [(usize, usize); n]
    }

    let mut a = Vec::with_capacity(n);
    let mut b = Vec::with_capacity(n);
    for (aa, bb) in ab {
        a.push(aa);
        b.push(bb);
    }
    a.sort_unstable();
    b.sort_unstable();

    let mut count = 0;
    let mut sum = 0;
    while sum < h && (!b.is_empty() && a.last().unwrap() < b.last().unwrap()) {
        sum += b.pop().unwrap();
        count += 1;
    }
    if h > sum {
        count += (h - sum + a.last().unwrap() - 1) / a.last().unwrap();
    }

    println!("{}", count);
}
