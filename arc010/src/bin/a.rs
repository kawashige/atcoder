use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: usize,
        b: usize,
        c: [usize; m]
    }

    let mut count = n;

    for i in 0..m {
        if count <= a {
            count += b;
        }

        if count < c[i] {
            println!("{}", i + 1);
            return;
        }
        count -= c[i];
    }
    println!("complete");
}
