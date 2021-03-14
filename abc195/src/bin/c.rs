use proconio::input;

fn main() {
    input! {
        n:u128
    }

    let mut count: u128 = 0;
    for i in 0..n.to_string().len() {
        if i != 0 && i % 3 == 0 {
            count += n - 10_u128.pow(i as u32) + 1;
        }
    }

    println!("{}", count);
}
