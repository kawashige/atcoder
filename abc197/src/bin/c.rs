use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    if n == 1 {
        println!("{}", a[0]);
        return;
    }

    let mut min = std::usize::MAX;
    for i in 1..(2_usize.pow(n as u32 - 1)) {
        let mut n1 = None;
        let mut n2 = 0;
        for j in 0..(n - 1) {
            n2 |= a[j];
            if (i & 1 << j) > 0 {
                if n1.is_none() {
                    n1 = Some(n2);
                } else {
                    n1 = Some(n1.unwrap() ^ n2);
                }
                n2 = 0;
            }
        }
        n2 |= a[n - 1];
        if n1.is_none() {
            n1 = Some(n2);
        } else {
            n1 = Some(n1.unwrap() ^ n2);
        }
        min = std::cmp::min(min, n1.unwrap());
    }
    println!("{}", min);
}
