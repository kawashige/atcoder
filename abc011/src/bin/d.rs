use proconio::input;

fn main() {
    input! {
        n: i32,
        d: i32,
        x: i32,
        y: i32
    }

    if x.abs() % d != 0 || y.abs() % d != 0 {
        println!("0");
        return;
    }

    let c_x = x.abs() / d;
    let c_y = y.abs() / d;

    if (n - c_x - c_y) % 2 != 0 {
        println!("0");
        return;
    }

    let mut factorial = vec![1; n as usize + 1];
    for i in 2..=(n as usize) {
        factorial[i] = factorial[i - 1] * i as u128;
    }

    let mut c = 0;
    let r = (n - c_x - c_y) / 2;

    for i in 0..=r {
        let j = r - i;
        let c1 = factorial[n as usize]
            / (factorial[(n - (c_x + i)) as usize] * factorial[(c_x + i) as usize]);
        let c2 = factorial[(n - (c_x + i)) as usize]
            / (factorial[(n - (c_x + i) - i) as usize] * factorial[i as usize]);
        let c3 = factorial[(n - (c_x + i) - i) as usize]
            / (factorial[(n - (c_x + i) - i - (c_y + j)) as usize] * factorial[(c_y + j) as usize]);
        c += c1 * c2 * c3;
    }

    let mut c = c as f64;
    for _ in 0..n {
        c /= 4.0;
    }

    println!("{}", c);
}
