use proconio::input;

fn main() {
    input! {
        n: usize,
        mut m: u64,
        mut ab: [(u64, u64); n]
    }

    ab.sort_unstable();

    let mut i = 0;
    let mut p = 0;
    while m > 0 {
        if m <= ab[i].1 {
            p += ab[i].0 * m;
            m = 0;
        } else {
            p += ab[i].0 * ab[i].1;
            m -= ab[i].1;
        }
        i += 1;
    }

    println!("{}", p);
}
