use proconio::input;

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

fn main() {
    input! {
        a: u64, b: u64, c: u64, d: u64
    }

    let c_count = b / c - (a + c - 1) / c + 1;
    let d_count = b / d - (a + d - 1) / d + 1;
    let lcm_cd = lcm(c, d);
    let cd_count = if lcm_cd <= b {
        b / lcm_cd - (a + lcm_cd - 1) / lcm_cd + 1
    } else {
        0
    };
    let count = b - a + 1 - c_count - d_count + cd_count;

    println!("{}", count);
}
