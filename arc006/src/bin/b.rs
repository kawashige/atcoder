use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.parse().ok().unwrap()
}

fn main() {
    let (n, l) = {
        let s = read::<String>();
        let mut sp = s.split_ascii_whitespace();
        (
            sp.next().unwrap().parse::<usize>().unwrap(),
            sp.next().unwrap().parse::<usize>().unwrap(),
        )
    };
    let board = (0..(l + 1))
        .map(|_| read::<String>().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut i = (0..board[l].len()).find(|i| board[l][*i] == 'o').unwrap() / 2;

    for j in (0..l).rev() {
        if i > 0 && board[j][i * 2 - 1] == '-' {
            i -= 1;
        } else if i < n - 1 && board[j][i * 2 + 1] == '-' {
            i += 1;
        }
    }

    println!("{}", i + 1);
}
