use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        c: usize
    }

    let mut min = 10000;
    for i in 0..10000 {
        let c_a = i * a;
        if c_a > n {
            break;
        }
        for j in 0..(10000 - i) {
            let sum = c_a + j * b;
            if sum > n {
                break;
            }
            if (n - sum) % c == 0 {
                min = std::cmp::min(min, i + j + (n - sum) / c);
            }
        }
    }

    println!("{}", min);
}
