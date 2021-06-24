use proconio::input;

fn main() {
    input! {
        n: u64
    }

    let mut count = 0;
    let mut x = n;
    for i in 2..=((n as f64).sqrt() as u64) {
        if x < i {
            break;
        }
        while x % i == 0 {
            count += 1;
            x /= i;
        }
    }
    if x > 1 {
        count += 1;
    }

    if count == 1 {
        println!("0");
    } else {
        let mut r = 0;
        if count % 2 == 1 {
            count += 1;
        }
        while count > 1 {
            count /= 2;
            if count != 1 && count % 2 == 1 {
                count += 1;
            }
            r += 1;
        }
        println!("{}", r);
    }
}
