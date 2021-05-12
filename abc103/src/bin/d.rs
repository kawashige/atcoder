use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut ab: [(usize, usize); m]
    }

    let mut v = vec![0; n + 1];

    ab.sort_unstable();
    for (a, b) in ab {
        v[b] = a;
    }

    let mut count = 0;
    let mut min = 1;
    for i in 2..=n {
        if v[i] >= min {
            count += 1;
            min = i;
        }
    }

    println!("{}", count);
}
