use proconio::input;

fn main() {
    input! {
        n: u128
    }

    let mut counts = [0; 10];
    let mut sum = 0;
    let mut count = 0;
    for c in n.to_string().chars() {
        count += 1;
        let i = c as usize - 0x30;
        sum += i;
        counts[i] += 1;
    }
    match sum % 3 {
        0 => println!("0"),
        1 => {
            if (0 < counts[1] || 0 < counts[4] || 0 < counts[7]) && count != 1 {
                println!("1");
            } else if 1 < counts[2] + counts[5] + counts[8] && count != 2 {
                println!("2");
            } else {
                println!("-1");
            }
        }
        2 => {
            if (0 < counts[2] || 0 < counts[5] || 0 < counts[8]) && count != 1 {
                println!("1");
            } else if 1 < counts[1] + counts[4] + counts[7] && count != 2 {
                println!("2");
            } else {
                println!("-1");
            }
        }
        _ => unreachable!(),
    }
}
