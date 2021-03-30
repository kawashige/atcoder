use proconio::input;

fn modinv(mut a: i64) -> i64 {
    let m = 1_000_000_007;
    let mut b = m;
    let mut u = 1;
    let mut v = 0;
    while b > 0 {
        let t = a / b;
        a -= t * b;
        std::mem::swap(&mut a, &mut b);
        u -= t * v;
        std::mem::swap(&mut u, &mut v);
    }
    u %= m;
    if u < 0 {
        u += m;
    }
    u
}

fn main() {
    input! {
        x: i64,
        y: i64
    }
    let factorial = (1..=std::cmp::max(x, y))
        .scan(1, |acc, i| Some(*acc * i % 1_000_000_007))
        .collect::<Vec<i64>>();
    println!("{:?}", factorial);

    let mut sum = 0;
    for i in 0..=x {
        if (x - i) % 2 != 0 || (x - 1) / 2 + 2 * i != y {
            continue;
        }

        let j = (x - 1) / 2;
        sum = if i == 0 || j == 0 {
            sum + 1
        } else {
            (sum + factorial[(i + j) as usize] * modinv(factorial[i as usize]) % 1_000_000_007
                * modinv(factorial[j as usize])
                % 1_000_000_007)
                % 1_000_000_007
        };
        println!("i: {}, j: {}, sum: {}", i, j, sum);
    }

    println!("{}", sum);
}
