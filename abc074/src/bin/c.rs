use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize,
        f: usize,
    }

    let mut result = vec![0; 2];
    result[0] = a * 100;
    let mut max = 0.0;
    for i in 0..=30 {
        if a * i * 100 > f {
            break;
        }
        for j in 0..30 {
            if (a * i + b * j) * 100 > f {
                break;
            }
            let water = a * i + b * j;
            let limit = water * e;
            for k in 0..=(limit / c) {
                if water * 100 + k * c > f {
                    break;
                }
                for l in 0..=(limit / d) {
                    if k * c + l * d > limit || water * 100 + k * c + l * d > f {
                        break;
                    }
                    let x = (100 * (k * c + l * d)) as f64
                        / (k * c + l * d + (a * i + b * j) * 100) as f64;

                    if max < x {
                        max = x;
                        result = vec![k * c + l * d + (a * i + b * j) * 100, k * c + l * d];
                    }
                }
            }
        }
    }

    println!("{} {}", result[0], result[1]);
}
