use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let s_n = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .sum::<usize>();

    if n % s_n == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
