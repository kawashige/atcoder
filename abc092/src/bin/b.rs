use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        x: usize,
        a: [usize; n]
    }

    let mut c = x;
    for num in a {
        c += (d - 1) / num + 1;
    }
    println!("{}", c);
}
