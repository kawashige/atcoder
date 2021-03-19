use proconio::input;

fn main() {
    input! {
        n: u64,
        k: u64
    }

    let mut i = 1;
    let mut num = k;
    while num <= n {
        i += 1;
        num *= k;
    }

    if n == 0 {
        println!("1");
    } else {
        println!("{}", i);
    }
}
