use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn main() {
    let _: usize = read();
    let mut nums: Vec<usize> = read::<String>()
        .split_whitespace()
        .map(|ss| ss.parse::<usize>().unwrap())
        .collect();
    nums.sort_unstable_by_key(|a| -(*a as i32));
    let (a, b) =
        nums.into_iter().enumerate().fold(
            (0, 0),
            |(a, b), (i, n)| {
                if i % 2 == 0 {
                    (a + n, b)
                } else {
                    (a, b + n)
                }
            },
        );
    println!("{}", a - b);
}
