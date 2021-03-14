use proconio::input;

fn gcd(a: usize, b: usize) -> usize {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}

fn main() {
    input! {
        k: usize
    }

    let mut sum: u64 = 0;
    for i in 1..=k {
        for j in i..=k {
            for l in j..=k {
                let n = gcd(gcd(i, j), l);
                sum += n as u64
                    * if i == j && j == l {
                        1
                    } else if i == j || j == l || i == l {
                        3
                    } else {
                        6
                    };
            }
        }
    }
    println!("{}", sum);
}
