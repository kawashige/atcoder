use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let count = (1_usize..=n)
        .filter(|i| !i.to_string().contains(&"7") && !to_oct(*i).contains(&"7"))
        .count();

    println!("{}", count);
}

fn to_oct(n: usize) -> String {
    let mut n = n;
    let mut s = String::new();
    while 0 < n {
        s = format!("{}{}", n % 8, s);
        n /= 8;
    }
    s
}
