use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize
    }

    if k == 0 {
        println!("{}", n * n);
        return;
    }

    let mut sum = 0;

    for i in (k + 1)..=n {
        let p = n / i;
        let r = n % i;
        sum += p * (i - k) + if r >= k { r - k + 1 } else { 0 };
    }

    println!("{}", sum);
}
