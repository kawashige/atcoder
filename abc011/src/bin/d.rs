use proconio::input;

fn main() {
    input! {
        n: usize,
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

    if (n as i32 - c_x - c_y) % 2 != 0 {
        println!("0");
        return;
    }

    let mut combination = vec![vec![0.0; n + 1]; n + 1];
    for i in 0..(n + 1) {
        combination[i][0] = 1.0 / 2.0_f64.powf(i as f64);
    }
    for i in 1..(n + 1) {
        for j in 1..i + 1 {
            combination[i][j] = combination[i - 1][j] + combination[i - 1][j - 1];
            combination[i][j] /= 2.0;
        }
    }

    let mut c = 0.0;
    let r = (n as i32 - c_x - c_y) / 2;

    for i in 0..=r {
        let j = r - i;
        c += combination[n][(c_x + i * 2) as usize]
            * combination[(c_x + i * 2) as usize][i as usize]
            * combination[n - (c_x + i * 2) as usize][j as usize];
    }

    println!("{}", c);
}
