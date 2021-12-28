use proconio::input;

fn main() {
    input! {
        n: u64
    }

    let mut sum = 0;
    let mut i = 1;

    while i <= n {
        sum += (n / i - n / (i + 1)) * i;
        if n / (i + 1) == 0 {
            break;
        }
        i = n / (n / (i + 1));
    }

    println!("{}", sum);
}
