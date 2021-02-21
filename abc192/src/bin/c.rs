use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize
    }

    let mut n: usize = n;
    for _ in 0..k {
        let mut digits = n.to_string().chars().collect::<Vec<char>>();
        digits.sort_unstable();
        let g1 = digits
            .iter()
            .rev()
            .collect::<String>()
            .trim_start_matches('0')
            .parse::<usize>()
            .unwrap_or(0);
        let g2 = digits
            .iter()
            .collect::<String>()
            .trim_start_matches('0')
            .parse::<usize>()
            .unwrap_or(0);
        n = g1 - g2;
        if n == 0 {
            break;
        }
    }
    println!("{}", n);
}
