use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }

    let mut count = 0;
    let mut sum = 0;
    for i in 0..n {
        sum += a[i];
        if i % 2 == 0 && sum <= 0 {
            count += sum.abs() + 1;
            sum = 1;
        } else if i % 2 == 1 && sum >= 0 {
            count += sum + 1;
            sum = -1;
        }
    }

    let mut count2 = 0;
    let mut sum = 0;
    for i in 0..n {
        sum += a[i];
        if i % 2 == 0 && sum >= 0 {
            count2 += sum + 1;
            sum = -1;
        } else if i % 2 == 1 && sum <= 0 {
            count2 += sum.abs() + 1;
            sum = 1;
        }
    }

    print!("{}", count.min(count2));
}
