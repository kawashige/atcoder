use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn main() {
    let nums: Vec<usize> = {
        let _: usize = read();
        read::<String>()
            .split_ascii_whitespace()
            .map(|ss| ss.parse::<usize>().unwrap())
            .collect()
    };
    let count = nums
        .into_iter()
        .map(|n| {
            let mut count = 0;
            let mut n = n;
            while n & 1 == 0 {
                n >>= 1;
                count += 1;
            }
            count
        })
        .min()
        .unwrap();
    println!("{}", count);
}
