use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn main() {
    let mut s: String = read();

    let mut changed = true;
    while changed && !s.is_empty() {
        changed = false;
        for target in vec!["dreamer", "eraser", "dream", "erase"] {
            if s.ends_with(target) {
                s = s.trim_end_matches(target).to_string();
                changed = true;
            }
        }
    }

    if s.is_empty() {
        println!("YES");
    } else {
        println!("NO");
    }
}
