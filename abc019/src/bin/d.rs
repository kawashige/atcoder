use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn main() {
    let n = read::<usize>();

    let mut max = 0;
    let mut max_v = 1;
    for i in 2..=n {
        println!("? {} {}", 1, i);

        let w = read::<usize>();

        if max < w {
            max = w;
            max_v = i;
        }
    }

    let mut max = 0;
    let mut max_v2 = 1;
    for i in 1..=n {
        if i == max_v {
            continue;
        }
        println!("? {} {}", max_v, i);

        let w = read::<usize>();

        if max < w {
            max = w;
            max_v2 = i;
        }
    }

    println!("? {} {}", max_v, max_v2);
    let w = read::<usize>();

    println!("! {}", w);
}
