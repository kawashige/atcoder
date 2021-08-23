use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut r = 0;
    for mut x in a {
        while x % 2 == 0 || x % 3 == 2 {
            x -= 1;
            r += 1;
        }
    }

    println!("{}", r);
}
