use proconio::input;

fn main() {
    input! {
        n: String,
        k: usize
    }

    let l = n.len();
    let digits = n
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();

    let mut sum = 0;
    if k == 1 {
        for i in 0..l {
            sum += if i == 0 { digits[0] } else { 9 };
        }
    } else if k == 2 {
        for i in 0..(l - 1) {
            for j in (i + 1)..l {
                sum += if i == 0 && digits[1..j].iter().all(|m| m == &0) {
                    (digits[0] - 1) * 9 + digits[j]
                } else if i == 0 {
                    digits[i] * 9
                } else {
                    9 * 9
                };
            }
        }
    } else {
        for i in 0..(l - 2) {
            for j in (i + 1)..(l - 1) {
                for k in (j + 1)..l {
                    sum += if i == 0
                        && digits[1..j].iter().all(|m| m == &0)
                        && digits[(j + 1)..k].iter().all(|m| m == &0)
                    {
                        if digits[j] == 0 || digits[k] == 0 {
                            0
                        } else {
                            (digits[0] - 1) * 9 * 9
                                + if digits[j] == 0 {
                                    0
                                } else {
                                    (digits[j] - 1) * 9
                                }
                                + digits[k]
                        }
                    } else if i == 0 && digits[1..j].iter().all(|m| m == &0) {
                        (digits[0] - 1) * 9 * 9 + digits[j] * 9
                    } else if i == 0 {
                        digits[0] * 9 * 9
                    } else {
                        9 * 9 * 9
                    };
                }
            }
        }
    }

    println!("{}", sum);
}
