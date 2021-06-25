use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let mut min = n.to_string().len();
    for i in 2..=((n as f64).sqrt() as usize) {
        if n % i == 0 {
            let f = std::cmp::max(i.to_string().len(), (n / i).to_string().len());
            min = std::cmp::min(min, f);
        }
    }

    println!("{}", min);
}
