use proconio::input;

fn main() {
    input! {
        mut p: usize
    }

    let mut num = vec![0; 11];
    num[1] = 1;
    for i in 2..=10 {
        num[i] = num[i - 1] * i;
    }

    let mut c = 0;
    for i in (1..=10).rev() {
        if num[i] <= p {
            let count = std::cmp::min(100, p / num[i]);
            p -= num[i] * count;
            c += count;
        }
    }

    println!("{}", c);
}
