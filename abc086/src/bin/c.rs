use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn main() {
    let n: usize = read();
    let points = (0..n)
        .map(|_| {
            read::<String>()
                .split_whitespace()
                .map(|ss| ss.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    for i in 0..points.len() {
        let distance = if i == 0 {
            points[0][1] + points[0][2]
        } else {
            (points[i][1] as i32 - points[i - 1][1] as i32).abs() as usize
                + (points[i][2] as i32 - points[i - 1][2] as i32).abs() as usize
        };
        let duration = if i == 0 {
            points[0][0]
        } else {
            points[i][0] - points[i - 1][0]
        };
        if duration < distance || (duration - distance) % 2 != 0 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
