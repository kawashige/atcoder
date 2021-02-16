use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let mut sum = 3;
    let mut count = 1;
    let mut i = 2;
    while sum <= n {
        if (n - sum) % i == 0 {
            count += 1;
        }
        i += 1;
        sum += i;
    }

    println!("{}", count * 2)
}
