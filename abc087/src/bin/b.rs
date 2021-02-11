use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn main() {
    let a: usize = read();
    let b: usize = read();
    let c: usize = read();
    let x: usize = read();

    let mut count = 0;
    for i in 0..=a {
        for j in 0..=b {
            for k in 0..=c {
                if x == 500 * i + 100 * j + 50 * k {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}
