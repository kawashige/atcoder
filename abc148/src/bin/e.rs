use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input! {
        n: u128
    }

    if n % 2 == 1 {
        println!("0");
        return;
    }

    let mut sum: u128 = 0;
    let mut num = 10;
    while num <= n {
        sum += n / num;
        num *= 5;
    }

    println!("{}", sum);
}
