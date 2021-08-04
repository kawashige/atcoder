use proconio::input;

fn main() {
    input! {
        mut n: usize
    }

    let mut sum = 0;
    let mut right = 1;
    let mut right2 = 0;

    while n > 0 {
        let r = n % 10;
        n /= 10;
        sum += if r == 0 {
            n * right
        } else if r == 1 {
            n * right + right2 + 1
        } else {
            (n + 1) * right
        };
        right2 += right * r;
        right *= 10;
    }

    println!("{}", sum);
}
