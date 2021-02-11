use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}
fn main() {
    let (n, y) = {
        let v = read::<String>()
            .split_whitespace()
            .map(|ss| ss.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        (v[0], v[1] / 1000)
    };
    for i in 0..=n {
        for j in 0..=(n - i) {
            if 10 * i + 5 * j + n - i - j == y {
                println!("{} {} {}", i, j, n - i - j);
                return;
            }
        }
    }
    println!("-1 -1 -1");
}
