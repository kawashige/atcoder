use proconio::input;

fn main() {
    input! {
        n: i32
    }

    let binary = (0..32)
        .map(|i| if n.abs() & 1 << i == 0 { 0 } else { 1 })
        .collect::<Vec<u8>>();
    let is_minus = n < 0;

    if n == 0 {
        println!("0");
        return;
    }

    let mut results = Vec::new();
    let mut carry = 0;
    for (i, b) in binary.into_iter().enumerate() {
        match (b, carry) {
            (0, 0) => {
                results.push(0);
            }
            (0, 1) | (1, 0) => {
                results.push(1);
                carry = if (i % 2 == 0 && is_minus) || (i % 2 == 1 && !is_minus) {
                    1
                } else {
                    0
                };
            }
            (1, 1) => {
                results.push(0);
                carry = 1;
            }
            _ => unreachable!(),
        }
    }

    while results.last() == Some(&0) {
        results.pop();
    }

    println!(
        "{}",
        results
            .iter()
            .rev()
            .map(|i| i.to_string())
            .collect::<String>()
    );
}
