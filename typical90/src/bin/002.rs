use proconio::input;

fn main() {
    input! {
        n: usize
    }

    for i in 1..2_usize.pow(n as u32) {
        let mut count = 0;
        let mut valid = true;
        for j in (0..n).rev() {
            if i & 1 << j == 0 {
                count += 1;
            } else {
                if count == 0 {
                    valid = false;
                    break;
                } else {
                    count -= 1;
                }
            }
        }

        if valid && count == 0 {
            println!(
                "{}",
                (0..n)
                    .rev()
                    .map(|j| if i & 1 << j == 0 { '(' } else { ')' })
                    .collect::<String>()
            );
        }
    }
}
