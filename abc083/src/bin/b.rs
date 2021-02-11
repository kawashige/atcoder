use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn main() {
    let (n, a, b) = {
        let v = read::<String>()
            .split_whitespace()
            .map(|ss| ss.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        (v[0], v[1], v[2])
    };
    let sum = (1..=n)
        .filter(|num| {
            let sum = num
                .to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .sum();
            a <= sum && sum <= b
        })
        .sum::<usize>();
    println!("{}", sum);
}
