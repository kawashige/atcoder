use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize
    }

    let mut sum = 0;
    for i in 1..=n {
        for j in 1..=k {
            let num = format!("{}0{}", i, j);
            sum += num.parse::<usize>().unwrap();
        }
    }

    println!("{}", sum);
}
