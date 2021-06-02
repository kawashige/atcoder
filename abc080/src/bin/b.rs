use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let x = n
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .sum::<usize>();

    if n % x == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
