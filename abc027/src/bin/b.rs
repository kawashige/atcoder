use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n]
    }

    let sum = a.iter().sum::<i32>();
    if sum % n as i32 != 0 {
        println!("-1");
        return;
    }
    let target = sum / n as i32;
    let mut remains = 0;
    let mut count = 0;

    for i in 0..n {
        remains += target - a[i];
        if remains != 0 {
            count += 1;
        }
    }

    println!("{}", count);
}
