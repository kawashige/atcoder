use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64
    }

    if a == b {
        println!("{}", a);
        return;
    }

    let mut result: u64 = 0;

    for i in 0..(64 - b.leading_zeros()) {
        let mut a_count = 2_u64.pow(i) * (a / 2_u64.pow(i + 1));
        if (a % 2_u64.pow(i + 1)) > 2_u64.pow(i) {
            a_count += (a % 2_u64.pow(i + 1)) - 2_u64.pow(i);
        }

        let mut b_count = 2_u64.pow(i) * ((b + 1) / 2_u64.pow(i + 1));
        if ((b + 1) % 2_u64.pow(i + 1)) > 2_u64.pow(i) {
            b_count += ((b + 1) % 2_u64.pow(i + 1)) - 2_u64.pow(i);
        }

        if (b_count - a_count) % 2 == 1 {
            result |= 1 << i;
        }
    }

    println!("{}", result);
}
