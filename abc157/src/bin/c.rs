use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        cond:  [(usize, i32); m]
    }

    let mut digits = vec![-1; n];

    for (s, c) in cond {
        if (digits[s - 1] > 0 && digits[s - 1] != c) || (n != 1 && s == 1 && c == 0) {
            println!("-1");
            return;
        }
        digits[s - 1] = c;
    }

    if n != 1 && digits[0] == -1 {
        digits[0] = 1;
    }

    let n = digits
        .into_iter()
        .map(|n| {
            if n == -1 {
                "0".to_string()
            } else {
                n.to_string()
            }
        })
        .collect::<String>();

    println!("{}", n);
}
