use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut count = 0;
    for x in a {
        let mut x = x;
        while x % 2 == 0 {
            count += 1;
            x /= 2;
        }
    }

    println!("{}", count);
}
