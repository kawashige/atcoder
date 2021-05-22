use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
    }

    let mut c_n = vec![0; n + 1];
    for x in c {
        c_n[x] += 1;
    }

    let mut b_n = vec![0; n + 1];
    for i in 0..n {
        b_n[b[i]] += c_n[i + 1];
    }

    let mut count: u64 = 0;
    for i in 0..n {
        count += b_n[a[i]] as u64;
    }

    println!("{}", count);
}
