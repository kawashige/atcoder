use proconio::input;
fn main() {
    input! {
        n: usize
    }

    let mut count = 0;
    for i in 1..n {
        let c = (i..=((n - 1) / i)).count();
        if c == 0 {
            break;
        }
        count += c * 2 - 1;
    }
    println!("{}", count);
}
