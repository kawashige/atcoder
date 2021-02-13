use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        tickets: Chars
    }

    let tickets = tickets
        .iter()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect::<Vec<i32>>();

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let mut num = tickets[0];
                num += if i == 0 { tickets[1] } else { -tickets[1] };
                num += if j == 0 { tickets[2] } else { -tickets[2] };
                num += if k == 0 { tickets[3] } else { -tickets[3] };
                if num == 7 {
                    println!(
                        "{}{}{}{}{}{}{}=7",
                        tickets[0],
                        if i == 0 { "+" } else { "-" },
                        tickets[1],
                        if j == 0 { "+" } else { "-" },
                        tickets[2],
                        if k == 0 { "+" } else { "-" },
                        tickets[3],
                    );
                    return;
                }
            }
        }
    }
}
