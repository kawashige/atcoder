use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        mut b: [usize; n],
        mut c: [usize; n],
    }

    a.sort_unstable();
    b.sort_unstable();
    c.sort_unstable();

    let mut b_counts = vec![0; n];
    let mut j = 0;
    for i in 0..n {
        while j < n && b[i] >= c[j] {
            j += 1;
        }
        b_counts[i] = (n - j) as u64;
    }

    let mut b_counts_acc = vec![0; n];
    b_counts_acc[n - 1] = b_counts[n - 1];
    for i in (0..(n - 1)).rev() {
        b_counts_acc[i] = b_counts[i] + b_counts_acc[i + 1];
    }

    let mut count: u64 = 0;
    let mut j = 0;
    for i in 0..n {
        while j < n && a[i] >= b[j] {
            j += 1;
        }
        if j == n {
            break;
        }
        count += b_counts_acc[j];
    }

    println!("{}", count);
}
