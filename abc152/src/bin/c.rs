use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n]
    }

    let mut count = 1;
    let mut min = p[0];

    for i in 1..n {
        if min >= p[i] {
            count += 1;
            min = p[i];
        }
    }

    println!("{}", count);
}
