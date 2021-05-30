use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u64,
        mut ab: [(u64, u64); n]
    }

    ab.sort_unstable();

    let mut i = 0;
    let mut k = k;
    for (a, b) in ab {
        if a - i > k {
            println!("{}", i + k);
            return;
        }
        k -= a - i;
        k += b;
        i = a;
    }

    println!("{}", i + k);
}
