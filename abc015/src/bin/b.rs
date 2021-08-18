use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut c = 0;
    let mut b = 0;
    for x in a {
        if x > 0 {
            c += 1;
            b += x;
        }
    }

    println!("{}", (b + c - 1) / c);
}
