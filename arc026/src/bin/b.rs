use proconio::input;

fn main() {
    input! {
        n: usize
    }

    if n == 1 {
        println!("Deficient");
        return;
    }

    let mut num = 1;
    for i in 2..=((n as f64).sqrt() as usize) {
        if n % i == 0 {
            num += i;
            if i != n / i {
                num += n / i;
            }
        }
    }

    if num == n {
        println!("Perfect");
    } else if num < n {
        println!("Deficient");
    } else {
        println!("Abundant");
    }
}
