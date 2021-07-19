use proconio::input;

fn main() {
    input! {
        n: u64,
        p: u64
    }

    if n == 1 {
        println!("{}", p);
        return;
    }

    let mut r = 1;
    let mut x = p;
    for i in 2..=((p as f64).sqrt() as u64) {
        if i > x {
            break;
        }
        let mut c = 0;
        while x % i == 0 {
            c += 1;
            x /= i;
        }

        while c >= n {
            c -= n;
            r *= i;
        }
    }

    println!("{}", r);
}
