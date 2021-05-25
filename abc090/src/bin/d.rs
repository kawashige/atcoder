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

    for i in k..n {
        sum += n - i;
    }

    for i in (k + 1)..n {
        // println!(
        //     "n: {}, i: {}, k + i: {}, n / i: {}, (k + i - 1) / i: {}, sum: {},",
        //     n,
        //     i,
        //     k + i,
        //     n / i,
        //     (k + i - 1) / i,
        //     n - (k + i) - (n / i - (k + i - 1) / i)
        // );
        sum += n / i * (i - k);
    }

    for i in k..n {
        sum -= n / i - (i + 1) / i;
    }

    println!("{}", sum);
}
