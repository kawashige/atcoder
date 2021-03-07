use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let mut num = n;
    let mut result = 0;
    for i in 2..=n {
        if i * i > num {
            break;
        }
        if num % i == 0 {
            let mut c = 0;
            let mut r = 1;
            while num % i == 0 {
                num /= i;
                r -= 1;
                if r == 0 {
                    c += 1;
                    r = c + 1;
                }
            }
            result += c;
        }
    }
    if num != 1 {
        result += 1;
    }

    println!("{}", result);
}
