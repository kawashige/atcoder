use proconio::input;

fn mod_pow(a: usize, b: usize, m: usize) -> usize {
    if b == 0 {
        1
    } else if b % 2 == 0 {
        let x = mod_pow(a, b / 2, m);
        x * x % m
    } else {
        a * mod_pow(a, b - 1, m) % m
    }
}

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize
    }

    let d = a.to_string().chars().last().unwrap().to_digit(10).unwrap() as usize;
    let mut digits = vec![d];
    while digits.len() == 1 || digits[0] != digits[digits.len() - 1] {
        let new_d = (digits[digits.len() - 1] * digits[0])
            .to_string()
            .chars()
            .last()
            .unwrap()
            .to_digit(10)
            .unwrap() as usize;
        digits.push(new_d);
    }
    digits.pop();

    println!(
        "{}",
        digits[(mod_pow(b, c, digits.len()) + digits.len() - 1) % digits.len()]
    );
}
