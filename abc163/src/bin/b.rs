use proconio::input;

fn main() {
    input! {
        n: u128,
        m: u128,
        a: [u128; m]
    }

    let sum = a.iter().sum::<u128>();
    if sum > n {
        println!("-1");
    } else {
        println!("{}", n - sum);
    }
}
