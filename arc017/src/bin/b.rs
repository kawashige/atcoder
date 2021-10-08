use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n]
    }

    if k == 1 {
        println!("{}", n);
        return;
    }

    let mut r = 0;
    let mut c = 1;

    for i in 1..n {
        if a[i - 1] < a[i] {
            c += 1;
            if c >= k {
                r += 1;
            }
        } else {
            c = 1;
        }
    }

    println!("{}", r);
}
