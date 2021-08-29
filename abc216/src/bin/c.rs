use proconio::input;

fn main() {
    input! {
        mut n: u64
    }

    let mut r = Vec::new();

    while 0 < n {
        if n % 2 == 0 {
            r.push('B');
            n /= 2;
        } else {
            r.push('A');
            n -= 1;
        }
    }

    println!("{}", r.into_iter().rev().collect::<String>());
}
