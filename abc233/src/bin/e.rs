use proconio::input;

fn main() {
    input! {
        x: String
    }

    let mut sum = x
        .chars()
        .map(|d| d.to_digit(10).unwrap() as usize)
        .sum::<usize>();

    let mut result = Vec::with_capacity(x.len());
    result.push((b'0' + (sum % 10) as u8) as char);
    let mut carry = sum / 10;

    for c in x.chars().rev() {
        sum -= c.to_digit(10).unwrap() as usize;
        result.push((b'0' + ((sum + carry) % 10) as u8) as char);
        carry = (sum + carry) / 10;
    }

    for c in carry.to_string().chars().rev() {
        result.push(c);
    }

    while result.last() == Some(&'0') {
        result.pop();
    }

    println!("{}", result.into_iter().rev().collect::<String>());
}
