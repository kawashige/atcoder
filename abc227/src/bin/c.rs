use proconio::input;

fn main() {
    input! {
        n: u64
    }

    let mut r = 0;
    for i in 1..=n {
        if n < i * i * i {
            break;
        }
        for j in i..=n {
            if n < i * j * j {
                break;
            }
            let max = n / (i * j);
            if j <= max {
                r += max - j + 1;
            }
        }
    }

    println!("{}", r);
}
