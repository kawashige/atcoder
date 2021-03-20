use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: Chars
    }

    let mut count = 0;
    let mut carry = 0;
    for i in (0..n.len()).rev() {
        let num = n[i].to_digit(10).unwrap() as u64 + carry;
        carry = 0;
        match num {
            0 => {}
            1 | 2 | 3 | 4 => {
                count += num;
            }
            5 => {
                if i != 0 && '5' <= n[i - 1] {
                    count += 5;
                    carry += 1;
                } else {
                    count += num;
                }
            }
            6 | 7 | 8 | 9 => {
                count += 10 - num;
                carry += 1;
            }
            10 => {
                carry = 1;
            }
            _ => unreachable!(),
        }
    }
    if carry > 0 {
        count += carry;
    }

    println!("{}", count);
}
