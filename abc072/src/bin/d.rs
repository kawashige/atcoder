use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n]
    }

    let mut i = 0;
    let mut count = 0;
    while i < n {
        if p[i] == i + 1 {
            i += 2;
            count += 1;
        } else {
            i += 1;
        }
    }

    println!("{}", count);
}
