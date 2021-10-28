use proconio::input;

fn main() {
    input! {
        l: usize,
        b: [usize; l]
    }

    let all = b.iter().fold(0, |acc, x| acc ^ x);
    let mut a_0 = 0;

    for i in 0..32 {
        let mut is_ng = true;
        let b = if all & 1 << i == 0 { 0 } else { 1 };
        if b ^ 0 == 0 {
            is_ng = false;
        } else if b ^ 1 == 1 {
            is_ng = false;
            a_0 |= 1 << i;
        }

        if is_ng {
            println!("-1");
            return;
        }
    }

    let mut a = a_0;
    println!("{}", a);
    for i in 0..(l - 1) {
        a ^= b[i];
        println!("{}", a);
    }
}
