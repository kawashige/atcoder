use proconio::input;

fn main() {
    input! {
        a: u64
    }

    for i in 10.. {
        let n = i
            .to_string()
            .chars()
            .map(|x| x.to_digit(10).unwrap() as u64)
            .fold(0, |acc, x| acc * i + x);

        if n == a {
            println!("{}", i);
            return;
        }
        if n > a {
            println!("-1");
            return;
        }
    }
}
