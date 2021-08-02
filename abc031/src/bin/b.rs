use proconio::input;

fn main() {
    input! {
        l: usize,
        h: usize,
        n: usize,
        a: [usize; n]
    }

    for x in a {
        if x < l {
            println!("{}", l - x);
        } else if h < x {
            println!("-1");
        } else {
            println!("0");
        }
    }
}
