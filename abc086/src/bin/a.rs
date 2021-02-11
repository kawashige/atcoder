use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn main() {
    let (a, b) = {
        let s: String = read();
        let mut ss = s.split_whitespace();
        (
            ss.next().unwrap().parse::<usize>().unwrap(),
            ss.next().unwrap().parse::<usize>().unwrap(),
        )
    };
    if (a * b) % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }
}
