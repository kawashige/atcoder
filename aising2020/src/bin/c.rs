use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let mut count = vec![0; n + 1];

    for x in 1..((n as f64).sqrt() as usize) {
        let n1 = x * x;
        for y in 1..((n as f64).sqrt() as usize) {
            let n2 = n1 + y * y + x * y;
            if n2 > n {
                continue;
            }
            for z in 1..((n as f64).sqrt() as usize) {
                let n3 = n2 + z * z + y * z + x * z;
                if n3 <= n {
                    count[n3] += 1;
                }
            }
        }
    }

    for i in 1..=n {
        println!("{}", count[i]);
    }
}
