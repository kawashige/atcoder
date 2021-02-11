use std::{collections::HashSet, str::FromStr};

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn main() {
    let n: usize = read();
    let nums: HashSet<usize> = (0..n).map(|_| read()).collect();
    println!("{}", nums.len());
}
